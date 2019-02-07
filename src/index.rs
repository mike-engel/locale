use http::StatusCode;
use now_lambda::{error::NowError, lambda, IntoResponse, Request, Response};
use std::error::Error;

fn handler(_: Request) -> Result<impl IntoResponse, NowError> {
    print!("Starting index request");
    print!("Building index response");
    let response = Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "text/html")
        .body("<!DOCTYPE html><html lang=en><meta charset=utf-8><meta content='initial-scale=1,width=device-width'name=viewport><title>Locale</title><link href=https://unpkg.com/tachyons@4.7.0/css/tachyons.min.css rel=stylesheet><style>.ww-ba{word-wrap:break-word}.ws-pw{white-space:pre-wrap}</style><body class='black-80 bg-washed-blue sans-serif'><noscript><img alt='fallback analytics script'src=https://analytics.mike-engel.com/api/a.gif></noscript><header class='black-80 bg-gold pv5 w-100'><div class='center mw9 w-90'><h1 class='mb2 mt0 f1 tc'>Locale</h1><h2 class='f4 fw4 mb0 tc'>A polyfill to understand your users' preferred languages</h2></div></header><main class='b--black-10 bb black-70 pv4'><div class='center mw9 w-90 cf'><section class='fl w-50-l w-50-m'><h2 class='f4 mb2 mt0'>How?</h2><p class='lh-copy measure'>Make a <code class='br1 bg-lightest-blue f6 normal pa1'>GET</code> request to <a class='blue link underline-hover'href=https://locale.now.sh/api>https://locale.now.sh/api</a> and get a sorted list of the user's languages.<h2 class='f4 mb2 mt4'>Why?</h2><p class='lh-copy measure'>In newer browsers you can gain access to an array of the user's preferred languages via <code class='br1 bg-lightest-blue f6 normal pa1'>window.navigator.languages</code>. In older browsers, however, that array isn't available. In comes locale, a server-side polyfill to get that list of languages.</section><section class='fl w-50-l w-50-m pl4-m pl5-l'><div><h3 class=f5>Using <code class='br1 bg-lightest-blue f6 normal pa1 ww-ba'>window.navigator.languages</code></h3><pre class='br1 ww-ba b--black-30 ba mid-gray pa2 ws-pw'><code data-hook=native></code></pre></div><div><h3 class=f5>Using <code class='br1 bg-lightest-blue f6 normal pa1 ww-ba'>locale.now.sh</code></h3><pre class='br1 ww-ba b--black-30 ba mid-gray pa2 ws-pw mb0'><code data-hook=polyfill>Loading...</code></pre></div></section></div></main><footer class='f6 black-70 center lh-copy mw9 pv4 w-90'><p class=mt0>Locale is open source and available on <a class='blue link underline-hover'href=https://github.com/mike-engel/locale>Github</a>. Are you fluent in another language and want to help translate? Let me know in a <a class='blue link underline-hover'href=https://github.com/mike-engel/locale/issues/new>Github issue</a>!<p class=mb0>Created with ♥ by <a class='blue link underline-hover'href=https://github.com/mike-engel>Mike Engel</a> using <a class='blue link underline-hover'href=https://rust-lang.org>Rust</a>, <a class='blue link underline-hover'href=https://rocket.rs>Rocket</a>, and <a class='blue link underline-hover'href=https://tachyons.io>Tachyons</a>. Hosted on Zeit's <a class='blue link underline-hover'href=https://now.sh>Now.sh</a>.</footer><script>var req=new XMLHttpRequest;req.addEventListener('load',function(e){if(200===e.target.status){var t=e.target.responseText;document.body.querySelector('[data-hook=\"polyfill\"]').innerText=t}}),req.open('GET','/api'),req.send(),document.body.querySelector('[data-hook=\"native\"]').innerText=JSON.stringify(navigator.languages)||'Not supported in your browser :('</script>")
        .expect("Internal Server Error");

    print!("About to send index response");
    Ok(response)
}

fn main() -> Result<(), Box<dyn Error>> {
    Ok(lambda!(handler))
}
