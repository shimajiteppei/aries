use js_sys::Uint8Array;
use nes_core::adapter::cartridge::CartridgeAdapter;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, Response};

pub struct CartridgeCtx {
    file_path: String,
    file_bytes: Vec<u8>,
}

impl CartridgeCtx {
    pub fn new(file_path: String) -> Self {
        Self {
            file_path,
            file_bytes: vec![],
        }
    }

    pub async fn fetch_file(&mut self) {
        let url = self.file_path.clone();
        let mut opts = RequestInit::new();
        opts.method("GET");
        let request = Request::new_with_str_and_init(&url, &opts).unwrap();

        let window = web_sys::window().unwrap();
        let resp_value = JsFuture::from(window.fetch_with_request(&request))
            .await
            .unwrap();
        let resp: Response = resp_value.dyn_into().unwrap();
        let downloaded_bytes =
            Uint8Array::new(&JsFuture::from(resp.array_buffer().unwrap()).await.unwrap()).to_vec();
        self.file_bytes = downloaded_bytes;
    }
}

impl CartridgeAdapter for CartridgeCtx {
    fn read_file(&self) -> Vec<u8> {
        self.file_bytes.to_vec()
    }
}
