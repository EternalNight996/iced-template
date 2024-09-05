use iced::{
  executor,
  widget::{button, column, container, progress_bar, text, Column},
  Alignment,
  Application,
  Command,
  Element,
  Length,
  Settings,
  Subscription,
  Theme,
};

pub fn main() -> iced::Result {
  Example::run(Settings::default())
}

#[derive(Debug)]
struct Example {
  downloads: Vec<Download>,
  last_id: usize,
}

#[derive(Debug, Clone)]
pub enum Message {
  Add,
  Download(usize),
  DownloadProgressed((usize, download::Progress)),
}

impl Application for Example {
  type Executor = executor::Default;
  type Flags = ();
  type Message = Message;
  type Theme = Theme;

  fn new(_flags: ()) -> (Example, Command<Message>) {
    (
      Example {
        downloads: vec![Download::new(0)],
        last_id: 0,
      },
      Command::none(),
    )
  }

  fn title(&self) -> String {
    String::from("Download progress - Iced")
  }

  fn update(&mut self, message: Message) -> Command<Message> {
    match message {
      Message::Add => {
        self.last_id += 1;

        self.downloads.push(Download::new(self.last_id));
      }
      Message::Download(index) => {
        if let Some(download) = self.downloads.get_mut(index) {
          download.start();
        }
      }
      Message::DownloadProgressed((id, progress)) => {
        if let Some(download) = self.downloads.iter_mut().find(|download| download.id == id) {
          download.progress(progress);
        }
      }
    };

    Command::none()
  }

  fn subscription(&self) -> Subscription<Message> {
    Subscription::batch(self.downloads.iter().map(Download::subscription))
  }

  fn view(&self) -> Element<'_, Message> {
    let downloads = Column::with_children(self.downloads.iter().map(Download::view))
      .push(button("Add another download").on_press(Message::Add).padding(10))
      .spacing(20)
      .align_items(Alignment::End);

    container(downloads)
      .width(Length::Fill)
      .height(Length::Fill)
      .center_x()
      .center_y()
      .padding(20)
      .into()
  }
}

#[derive(Debug)]
struct Download {
  id: usize,
  state: State,
}

#[derive(Debug)]
enum State {
  Idle,
  Downloading { progress: f32 },
  Finished,
  Errored,
}

impl Download {
  pub fn new(id: usize) -> Self {
    Download { id, state: State::Idle }
  }

  pub fn start(&mut self) {
    match self.state {
      State::Idle { .. } | State::Finished { .. } | State::Errored { .. } => {
        self.state = State::Downloading { progress: 0.0 };
      }
      State::Downloading { .. } => {}
    }
  }

  pub fn progress(&mut self, new_progress: download::Progress) {
    if let State::Downloading { progress } = &mut self.state {
      match new_progress {
        download::Progress::Started => {
          *progress = 0.0;
        }
        download::Progress::Advanced(percentage) => {
          *progress = percentage;
        }
        download::Progress::Finished => {
          self.state = State::Finished;
        }
        download::Progress::Errored => {
          self.state = State::Errored;
        }
      }
    }
  }

  pub fn subscription(&self) -> Subscription<Message> {
    match self.state {
      State::Downloading { .. } => {
        download::file(self.id, "https://v95-sz-web-prime.douyinvod.com/video/tos/cn/tos-cn-ve-15/oo0A1EqDghsB9c5FfKm9AiAlQhegZXIkmCBiAD/?a=6383&br=918&bt=918&btag=c0000e00038000&cd=0%7C0%7C0%7C3&ch=5&cquery=101s_100B_100x_100z_100o&cr=3&cs=0&cv=1&dr=0&ds=6&dy_q=1723532184&expire=1723543751&feature_id=f0150a16a324336cda5d6dd0b69ed299&ft=52zLTKWZQQqUYqfuIrGJC5qSYiAdSjDtGEZgsCAq8_45a&is_ssr=1&l=202408131456231E15908C596CBC03C08A&lr=all&mime_type=video_mp4&ply_type=4&policy=4&qs=0&rc=ZTNkZ2g5aTo8ZzwzODY4PEBpamp1Zmw5cjM5czMzNGkzM0BeYmNhLzNgNi0xMV5iYjNeYSNubTBsMmRrYmNgLS1kLTBzcw%3D%3D&signature=872269156b32a3053d58853c9c6e1c5a&tk=webid&webid=3c3e9d4a635845249e00419877a3730e2149197a63ddb1d8525033ea2b3354c2da68f52244f0fcac05f8374a2c34ffd1376015dc9fa79494759ed7d7e6f3698d8c737a1cbc6d72a8b6dfebdd791547303092cf63ad813f7d8221883c00fd865e7f1a80ca7f12072cceca1e7b977d47353e0b64b6d60ec2faed9f02260a5afd5cc6567966682d3ec6bf3e00cbfc641ba14eee96af0ebe9d68c9be7cd5dfc32937-7dd05c05635c7d5fb92646dfe94d65c5&fid=91441482fdca3a9fbd49f74419afdd82").map(Message::DownloadProgressed)
      }
      _ => Subscription::none(),
    }
  }

  pub fn view(&self) -> Element<'_, Message> {
    let current_progress = match &self.state {
      State::Idle { .. } => 0.0,
      State::Downloading { progress } => *progress,
      State::Finished { .. } => 100.0,
      State::Errored { .. } => 0.0,
    };

    let progress_bar = progress_bar(0.0..=100.0, current_progress);

    let control: Element<_> = match &self.state {
      State::Idle => {
        button("Start the download!")
          .on_press(Message::Download(self.id))
          .into()
      }
      State::Finished => {
        column!["Download finished!", button("Start again")]
          .spacing(10)
          .align_items(Alignment::Center)
          .into()
      }
      State::Downloading { .. } => text(format!("Downloading... {current_progress:.2}%")).into(),
      State::Errored => {
        column![
          "Something went wrong :(",
          button("Try again").on_press(Message::Download(self.id)),
        ]
        .spacing(10)
        .align_items(Alignment::Center)
        .into()
      }
    };

    Column::new()
      .spacing(10)
      .padding(10)
      .align_items(Alignment::Center)
      .push(progress_bar)
      .push(control)
      .into()
  }
}

mod download {
  use std::hash::Hash;

  use iced::subscription;

  // Just a little utility function
  pub fn file<I: 'static + Hash + Copy + Send + Sync, T: ToString>(
    id: I,
    url: T,
  ) -> iced::Subscription<(I, Progress)> {
    subscription::unfold(id, State::Ready(url.to_string()), move |state| download(id, state))
  }

  async fn download<I: Copy>(id: I, state: State) -> ((I, Progress), State) {
    match state {
      State::Ready(url) => {
        let response = reqwest::get(&url).await;

        match response {
          Ok(response) => {
            if let Some(total) = response.content_length() {
              (
                (id, Progress::Started),
                State::Downloading {
                  response,
                  total,
                  downloaded: 0,
                },
              )
            }
            else {
              ((id, Progress::Errored), State::Finished)
            }
          }
          Err(_) => ((id, Progress::Errored), State::Finished),
        }
      }
      State::Downloading {
        mut response,
        total,
        downloaded,
      } => {
        match response.chunk().await {
          Ok(Some(chunk)) => {
            let downloaded = downloaded + chunk.len() as u64;

            let percentage = (downloaded as f32 / total as f32) * 100.0;

            (
              (id, Progress::Advanced(percentage)),
              State::Downloading {
                response,
                total,
                downloaded,
              },
            )
          }
          Ok(None) => ((id, Progress::Finished), State::Finished),
          Err(_) => ((id, Progress::Errored), State::Finished),
        }
      }
      State::Finished => {
        // We do not let the stream die, as it would start a
        // new download repeatedly if the user is not careful
        // in case of errors.
        iced::futures::future::pending().await
      }
    }
  }

  #[derive(Debug, Clone)]
  pub enum Progress {
    Started,
    Advanced(f32),
    Finished,
    Errored,
  }

  pub enum State {
    Ready(String),
    Downloading {
      response: reqwest::Response,
      total: u64,
      downloaded: u64,
    },
    Finished,
  }
}
