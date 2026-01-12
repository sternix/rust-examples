/*
[dependencies]
tokio = { version = "0.2", features = ["macros", "rt-threaded"] }
warp = "0.2"
futures = { version = "0.3", default-features = false, features = ["alloc"] }
bytes = "0.5"
*/

use bytes::BufMut;
use futures::{TryFutureExt, TryStreamExt};
use std::convert::Infallible;
use warp::Filter;

use warp::http::Response;

#[tokio::main]
async fn main() {
    let get = warp::get().map(|| Response::builder().body(get_body()));

    let post = warp::path("upload")
        .and(warp::body::content_length_limit(1024 * 1024 * 1024))
        .and(warp::multipart::form().max_length(1024 * 1024 * 1024))
        .and_then(upload_file);

    let routes = get.or(post);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

fn get_body() -> String {
    let s = r#"
<!doctype html>
<html>
<head>
<title>Form Örneği</title>
<meta charset="utf-8">
</head>
<body>

<form name="frmUpload" method="post" enctype="multipart/form-data" action="/upload">
Adı: <input type="text" name="adi"/>
Resim: <input type="file" name="dosya"/>
<input type="submit" value="Kaydet" />
</form>

</body>
</html>
	"#;

    s.to_string()
}

async fn upload_file(form: warp::multipart::FormData) -> Result<impl warp::Reply, Infallible> {
    let _part: Result<Vec<(String, Vec<u8>)>, warp::Rejection> = form
        .and_then(|part| {
            let name = part.name().to_string();
            let value = part.stream().try_fold(Vec::new(), |mut vec, data| {
                vec.put(data);
                async move { Ok(vec) }
            });
            value.map_ok(move |vec| (name, vec))
        })
        .try_collect()
        .await
        .map_err(|e| {
            panic!("multipart error: {:?}", e);
        });

    Ok("UPLOAD")
}
