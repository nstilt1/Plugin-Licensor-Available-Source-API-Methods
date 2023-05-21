mod my_modules;

use my_modules::{
    networking::{
        input::{
            request::MagicalProcess,
            encrypted::Encrypted,
        },
        output::{
            response::*,
        },
    },
    utils::{utils::*, to_json::ToJson},
};

use lambda_http::{run, service_fn, Body, Error, Request, Response};


async fn function_handler(event: Request) -> Result<Response<Body>, Error> {

    let request_json = event.extract_json();
    if request_json.as_ref().is_err() {
        return request_json.unwrap_err().respond();
    }

    // decrypt request
    let encrypted_request = Encrypted::new(&request_json.unwrap());
    if encrypted_request.as_ref().is_err() {
        return encrypted_request.unwrap_err().respond();
    }
    let decrypted_request_result = encrypted_request.unwrap().decrypt().await;
    if decrypted_request_result.as_ref().is_err() {
        return decrypted_request_result.unwrap_err().respond();
    }
    let mut decrypted_request = decrypted_request_result.unwrap();
    // error below here

    // prepare user item and license item
    let user_item = decrypted_request.prepare_to_license().await;
    // error in prepare_to_license
    //return error_resp(500, "Error in .update_license");
    let update_license_result = decrypted_request.update_license_data(user_item).await;
    if update_license_result.as_ref().is_err() {
        return update_license_result.unwrap_err().respond();
    }

    // error above here
    //return error_resp(500, "Made it to 43");

    let send_result = decrypted_request.batch_write().await;
    if send_result.as_ref().is_err(){return send_result.unwrap_err().respond();}

    // the database has been updated
    // format response
    // maybe one day, make this return the new license info
    let response_prep = HttpResponse::new(decrypted_request);
    if response_prep.as_ref().is_err() {return response_prep.unwrap_err()._202("CLM51g").respond();}
    let response_json = response_prep.unwrap().to_json();
    if response_json.as_ref().is_err() {return response_json.unwrap_err()._202("CLM53j").respond();}
    let json = response_json.unwrap();
    
    return success_resp(&json);

}

#[tokio::main]
async fn main() -> Result<(), Error> {
    std::env::set_var("RUST_BACKTRACE", "1");
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // disable printing the name of the module in every log line.
        .with_target(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    run(service_fn(function_handler)).await
}
