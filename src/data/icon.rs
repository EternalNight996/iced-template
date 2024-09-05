#![allow(unused)]
use iced::alignment;
use iced::widget::{image, text, Image};

use crate::config::theme;
use crate::ui::components::Text;

use super::IMG_MAP;
/// 日志
pub fn log<'a>() -> Text<'a> {
  icon('\u{e715}')
}
/// 新窗口
pub fn new_window<'a>() -> Text<'a> {
  icon('\u{e601}')
}
/// 下载 download cloud
pub fn download_cloud<'a>() -> Text<'a> {
  icon('\u{ec1d}')
}
/// 下载 download
pub fn download<'a>() -> Text<'a> {
  icon('\u{e63c}')
}
/// 上传 upload cloud
pub fn upload_cloud<'a>() -> Text<'a> {
  icon('\u{e621}')
}
/// 上传 upload
pub fn upload<'a>() -> Text<'a> {
  icon('\u{e646}')
}
/// 检查圆圈 check-circle 🌟
pub fn check_circle<'a>() -> Text<'a> {
  icon('\u{e77d}')
}
/// 关闭圆圈 close-circle 🚫
pub fn close_circle<'a>() -> Text<'a> {
  icon('\u{e77e}')
}
/// 信息圆圈 info-circle ℹ️
pub fn info_circle<'a>() -> Text<'a> {
  icon('\u{e77f}')
}
/// 减号圆圈 minus-circle ⑃
pub fn minus_circle<'a>() -> Text<'a> {
  icon('\u{e780}')
}
/// 加号圆圈 plus-circle ⑊
pub fn plus_circle<'a>() -> Text<'a> {
  icon('\u{e781}')
}
/// 播放圆圈 play-circle ►
pub fn play_circle<'a>() -> Text<'a> {
  icon('\u{e782}')
}
/// 暂停 pause
pub fn pause<'a>() -> Text<'a> {
  icon('\u{e783}')
}
/// 同步 sync 🔃
pub fn sync<'a>() -> Text<'a> {
  icon('\u{e786}')
}
/// 撤销 undo ↶
pub fn undo<'a>() -> Text<'a> {
  icon('\u{e787}')
}
/// 重做 redo ↷
pub fn redo<'a>() -> Text<'a> {
  icon('\u{e788}')
}
/// 重新加载 reload 🔁
pub fn reload<'a>() -> Text<'a> {
  icon('\u{e648}')
}
/// 关闭电源 poweroff 🚫
pub fn poweroff<'a>() -> Text<'a> {
  icon('\u{e78c}')
}
/// 退出 logout 🔓
pub fn logout<'a>() -> Text<'a> {
  icon('\u{e78d}')
}
/// 设置 setting 🔧
pub fn setting<'a>() -> Text<'a> {
  icon('\u{e78e}')
}
/// 编辑方块 edit-square 📝
pub fn edit_square<'a>() -> Text<'a> {
  icon('\u{e791}')
}
/// 导出 export 📤
pub fn export<'a>() -> Text<'a> {
  icon('\u{e792}')
}
/// 保存 save 💾
pub fn save<'a>() -> Text<'a> {
  icon('\u{e67c}')
}
/// 保存 save2 💾
pub fn save2<'a>() -> Text<'a> {
  icon('\u{e618}')
}

/// 导入 Import 📥
pub fn import<'a>() -> Text<'a> {
  icon('\u{e794}')
}
/// 应用商店 app store 📱
pub fn appstore<'a>() -> Text<'a> {
  icon('\u{e795}')
}
/// 布局 layout 📐
pub fn layout<'a>() -> Text<'a> {
  icon('\u{e796}')
}
/// 控制 control 🎛
pub fn control<'a>() -> Text<'a> {
  icon('\u{e797}')
}
/// 添加用户 add user 👤
pub fn adduser<'a>() -> Text<'a> {
  icon('\u{e7ae}')
}
/// 删除团队 delete team 🗑️
pub fn deleteteam<'a>() -> Text<'a> {
  icon('\u{e7af}')
}
/// 删除用户 delete user 👥
pub fn deleteuser<'a>() -> Text<'a> {
  icon('\u{e7b0}')
}
/// 添加团队 addteam 🏢
pub fn addteam<'a>() -> Text<'a> {
  icon('\u{e7b1}')
}
/// 用户 user 👨‍💻
pub fn user<'a>() -> Text<'a> {
  icon('\u{e7b2}')
}
/// 团队 team 🏆
pub fn team<'a>() -> Text<'a> {
  icon('\u{e7b3}')
}
/// 删除 delete 🗑️
pub fn delete<'a>() -> Text<'a> {
  icon('\u{e7c3}')
}
/// 休息 rest 🛌
pub fn rest<'a>() -> Text<'a> {
  icon('\u{e7c4}')
}
/// 过滤器 filter 🔍
pub fn filter<'a>() -> Text<'a> {
  icon('\u{e7c7}')
}
/// 文件夹 folder 📁
pub fn folder<'a>() -> Text<'a> {
  icon('\u{e7d1}')
}
/// 文件夹打开 folder-open 📂
pub fn folder_open<'a>() -> Text<'a> {
  icon('\u{e7d2}')
}
/// 文件夹添加 folder-add 📁
pub fn folder_add<'a>() -> Text<'a> {
  icon('\u{e7d3}')
}
/// 停止 stop ⏹
pub fn stop<'a>() -> Text<'a> {
  icon('\u{e842}')
}
/// 主页 home 🏠
pub fn home<'a>() -> Text<'a> {
  icon('\u{e69b}')
}
/// 打印 print 🖨️
pub fn print<'a>() -> Text<'a> {
  icon('\u{e67a}')
}
/// 主题 theme 📅
pub fn theme<'a>() -> Text<'a> {
  icon('\u{e600}')
}
/// 错误 error
pub fn error<'a>() -> Text<'a> {
  icon('\u{e6b1}').style(theme::Text::Error)
}
/// 警告 warn
pub fn warn<'a>() -> Text<'a> {
  icon('\u{e682}').style(theme::Text::Warning)
}
/// 成功 success
pub fn success<'a>() -> Text<'a> {
  icon('\u{e67f}').style(theme::Text::Success)
}
/// 等待 wait
pub fn wait<'a>() -> Text<'a> {
  icon('\u{e816}')
}
/// 时间 time
pub fn time<'a>() -> Text<'a> {
  icon('\u{e61d}')
}
fn icon(unicode: char) -> Text<'static> {
  text(unicode.to_string())
    .font(iced::Font::with_name("iconfont"))
    .size(25)
    .horizontal_alignment(alignment::Horizontal::Center)
    .vertical_alignment(alignment::Vertical::Center)
}

pub fn main_logo() -> Image<image::Handle> {
  image(get_img("main"))
}
pub fn title_logo() -> Image<image::Handle> {
  image(get_img("title"))
}

fn get_img(src: &str) -> image::Handle {
  IMG_MAP
    .get(src)
    .unwrap_or_else(|| panic!("invalid key '{src}' provided"))
    .clone()
}
