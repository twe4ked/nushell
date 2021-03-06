use crate::format::RenderView;
use crate::prelude::*;
use derive_new::new;

// A list is printed one line at a time with an optional separator between groups
#[allow(unused)]
#[derive(new)]
pub struct ListView {
    list: Vec<Vec<String>>,
    sep: String,
}

impl RenderView for ListView {
    fn render_view(&self, host: &mut dyn Host) -> Result<(), ShellError> {
        for output in &self.list {
            let string: String = output.iter().map(|l| format!("{}\n", l)).collect();
            host.stdout(&format!("{}{}", string, self.sep));
        }

        Ok(())
    }
}
