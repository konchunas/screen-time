use gio::Icon;

pub trait AppInfoProvider {
    fn get_name(&self) -> Option<String>;
    fn get_icon(&self) -> Option<Icon>;
    fn get_categories(&self) -> Option<String>;
}
