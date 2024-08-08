use url::Url;

use crate::client::{Auth, Client};

#[derive(Clone, Debug)]
pub struct FileManager<'a> {
    pub client: &'a Client<Auth>,
}

pub struct FileUrlRequestBulder<'a> {
    pub(crate) client: &'a Client<Auth>,
    pub(crate) collection_name: &'a str,
    pub(crate) identifier: &'a str,
    pub(crate) file_name: &'a str,
    pub(crate) thumb: Option<String>,
    pub(crate) download: bool,
    pub(crate) token: Option<String>,
}

impl<'a> FileUrlRequestBulder<'a> {
    pub fn download(mut self, download: bool) -> Self {
        self.download = download;
        self
    }
    pub fn thumb(mut self, thumb: String) -> Self {
        self.thumb = Some(thumb);
        self
    }
    pub fn token(mut self, token: String) -> Self {
        self.token = Some(token);
        self
    }
    pub fn call(&self) -> String {
        let url = format!(
            "{}/api/files/{}/{}/{}",
            self.client.base_url, self.collection_name, self.identifier, self.file_name
        );

        if let Ok(mut parsed_url) = Url::parse(&url) {
            {
                let mut query_pairs = parsed_url.query_pairs_mut();

                if self.download {
                    query_pairs.append_pair("download", "1");
                }

                if let Some(ref thumb) = self.thumb {
                    query_pairs.append_pair("thumb", thumb);
                }
                if let Some(ref token) = self.token {
                    query_pairs.append_pair("token", token);
                }
            }

            return parsed_url.to_string();
        }

        url
    }
}
