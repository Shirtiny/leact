use dioxus::{
    hooks::{use_context, use_context_provider, use_memo, use_signal},
    signals::{Memo, Readable, Signal},
};

#[derive(Clone)]
pub struct FileState {
    pub file: Signal<Option<web_sys::File>>,
    pub url: Memo<String>, // 添加生命周期标注
}

pub fn use_file_provider() -> FileState {
    let file: Signal<Option<web_sys::File>> = use_signal(|| None);

    let url = use_memo(move || {
        let object_url = {
            let file_ref = file.read();
            if let Some(blob) = file_ref.as_ref() {
                web_sys::Url::create_object_url_with_blob(blob).unwrap()
            } else {
                String::new()
            }
        };

        return object_url;
    });

    let state: FileState = use_context_provider(|| FileState { file, url });

    return state;
}

pub fn use_file() -> FileState {
    let FileState { file, url } = use_context::<FileState>();
    
    
    
    return FileState { file, url };
}
