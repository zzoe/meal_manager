use xilem::masonry::parley::style::FontStack;

pub const CJK_FONT_STACK: FontStack<'static> =
    FontStack::Source(std::borrow::Cow::Borrowed("PingFang SC"));

pub mod app_logic;
pub mod pages;
