use crate::ToolError;

pub trait Tool {
    fn mount(&mut self, host_element: web_sys::HtmlElement) -> Result<(), ToolError>;
    fn unmount(&mut self) -> Result<(), ToolError>;
}
