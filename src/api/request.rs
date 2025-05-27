#[derive(Debug, Clone)]
pub struct Request {
    server: String,
    project_iri: String,
    offset_nr: usize,
    token: String,
}

impl Request {
    pub fn get_trig(&mut self) -> String {
        // https://docs.dasch.swiss/latest/DSP-API/03-endpoints/api-admin/projects/#get-all-data-of-a-project
        //curl --request GET \
        //   --url http://localhost:3333/admin/projects/iri/http%3A%2F%2Frdfh.ch%2Fprojects%2F00FF/AllData \
        //   --header 'Authorization: Basic cm9vdEBleGFtcGxlLmNvbTp0ZXN0'
        format!("https://api.{}.dasch.swiss/admin/projects/iri/{}/AllData --header 'Authorization: Basic {}'", self.server, self.project_iri, self.token)
    }
    pub(crate) fn offset_plus_one(&mut self) {
        self.offset_nr += 1;
    }
}

pub fn new(server: String, project_iri: String, token: String) -> Request {
    /// https://url.spec.whatwg.org/#fragment-percent-encode-set
    ///
    /*
    const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<').add(b'>').add(b'`');

    let iter = utf8_percent_encode(label.as_str(), FRAGMENT);
    let encoded: String = iter.collect();

     */
    Request {
        server: server.to_string(),
        project_iri,
        offset_nr: 0,
        token,
    }
}
