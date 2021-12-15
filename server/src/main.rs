use std::{
    fs::File,
    io::{stdout, BufRead, BufReader},
    net::{Ipv4Addr, SocketAddr},
    sync::Arc,
};

use dropshot::{
    endpoint, ApiDescription, ConfigDropshot, ConfigLogging, ConfigLoggingLevel, EmptyScanParams,
    HttpError, HttpResponseOk, HttpServerStarter, PaginationParams, Query, RequestContext,
    ResultsPage,
};

#[endpoint {
    method = GET,
    path = "/words"
}]
async fn list_words(
    rqctx: Arc<RequestContext<Vec<String>>>,
    query: Query<PaginationParams<EmptyScanParams, String>>,
) -> Result<HttpResponseOk<ResultsPage<String>>, HttpError> {
    let pag_params = query.into_inner();
    let limit = rqctx.page_limit(&pag_params)?.get() as usize;
    let all_words = rqctx.context();

    let words: Vec<String> = match &pag_params.page {
        dropshot::WhichPage::First(_) => all_words.iter().take(limit).cloned().collect(),
        dropshot::WhichPage::Next(last) => {
            let index = match all_words.binary_search(last) {
                Ok(i) => i + 1,
                Err(i) => i,
            };

            all_words.iter().skip(index).take(limit).cloned().collect()
        }
    };

    Ok(HttpResponseOk(ResultsPage::new(
        words,
        &EmptyScanParams {},
        |s, _| s.clone(),
    )?))
}

#[tokio::main]
async fn main() -> Result<(), String> {
    let mut api = ApiDescription::new();
    api.register(list_words).unwrap();

    api.openapi("pagination-demo", "9000")
        .write(&mut stdout())
        .unwrap();

    let mut words = BufReader::new(File::open("/usr/share/dict/words").unwrap())
        .lines()
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    words.sort();

    let config_dropshot = ConfigDropshot {
        bind_address: SocketAddr::from((Ipv4Addr::LOCALHOST, 8080)),
        request_body_max_bytes: 1024,
        tls: None,
    };
    let config_logging = ConfigLogging::StderrTerminal {
        level: ConfigLoggingLevel::Debug,
    };
    let log = config_logging
        .to_logger("example-pagination-basic")
        .map_err(|error| format!("failed to create logger: {}", error))?;
    let server = HttpServerStarter::new(&config_dropshot, api, words, &log)
        .map_err(|error| format!("failed to create server: {}", error))?
        .start();
    server.await
}
