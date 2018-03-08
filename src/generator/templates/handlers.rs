pub static TEXT: &'static str = "use std::io::Cursor;
use std::convert::From;

use diesel::result::Error;

use rocket::Catcher;
use rocket::http::{Status, ContentType};
use rocket::request::Request;
use rocket::response::{Response, Responder};
use rocket_contrib::JsonValue;

/// Put all of your catchers here
pub fn init_catchers() -> Vec<Catcher> {
    catchers![
        bad_request_handler,
        unauthorized_handler,
        forbidden_handler,
        not_found_handler,
        internal_server_error_handler,
        service_unavailable_handler,
    ]
}

#[catch(400)]
fn bad_request_handler() -> APIResponse {
    bad_request()
}

#[catch(401)]
fn unauthorized_handler() -> APIResponse {
    unauthorized()
}

#[catch(403)]
fn forbidden_handler() -> APIResponse {
    forbidden()
}

#[catch(404)]
fn not_found_handler() -> APIResponse {
    not_found()
}

#[catch(500)]
fn internal_server_error_handler() -> APIResponse {
    internal_server_error()
}

#[catch(503)]
fn service_unavailable_handler() -> APIResponse {
    service_unavailable()
}

#[derive(Debug)]
pub struct APIResponse {
    data: JsonValue,
    status: Status,
}

impl APIResponse {
    /// Set the data of the `Response` to `data`.
    pub fn data(mut self, data: JsonValue) -> APIResponse {
        self.data = data;
        self
    }

    /// Convenience method to set `self.data` to `{\"message\": message}`.
    pub fn message(mut self, message: &str) -> APIResponse {
        self.data = json!({
            \"message\": message
        });
        self
    }
}

impl From<Error> for APIResponse {
    fn from(_: Error) -> Self {
        internal_server_error()
    }
}

impl<'r> Responder<'r> for APIResponse {
    fn respond_to(self, _req: &Request) -> Result<Response<'r>, Status> {
        let body = self.data;

        Response::build()
            .status(self.status)
            .sized_body(Cursor::new(body.to_string()))
            .header(ContentType::JSON)
            .ok()
    }
}

pub fn ok() -> APIResponse {
    APIResponse {
        data: json!(null),
        status: Status::Ok,
    }
}

pub fn created() -> APIResponse {
    APIResponse {
        data: json!(null),
        status: Status::Created,
    }
}

pub fn accepted() -> APIResponse {
    APIResponse {
        data: json!(null),
        status: Status::Accepted,
    }
}

pub fn no_content() -> APIResponse {
    APIResponse {
        data: json!(null),
        status: Status::NoContent,
    }
}

pub fn bad_request() -> APIResponse {
    APIResponse {
        data: json!({\"message\": \"Bad Request\"}),
        status: Status::BadRequest,
    }
}

pub fn unauthorized() -> APIResponse {
    APIResponse {
        data: json!({\"message\": \"Unauthorized\"}),
        status: Status::Unauthorized,
    }
}

pub fn forbidden() -> APIResponse {
    APIResponse {
        data: json!({\"message\": \"Forbidden\"}),
        status: Status::Forbidden,
    }
}

pub fn not_found() -> APIResponse {
    APIResponse {
        data: json!({\"message\": \"Not Found\"}),
        status: Status::NotFound,
    }
}

pub fn method_not_allowed() -> APIResponse {
    APIResponse {
        data: json!({\"message\": \"Method Not Allowed\"}),
        status: Status::MethodNotAllowed,
    }
}

pub fn conflict() -> APIResponse {
    APIResponse {
        data: json!({\"message\": \"Conflict\"}),
        status: Status::Conflict,
    }
}

pub fn unprocessable_entity(errors: JsonValue) -> APIResponse {
    APIResponse {
        data: json!({\"message\": *errors}),
        status: Status::UnprocessableEntity,
    }
}

pub fn internal_server_error() -> APIResponse {
    APIResponse {
        data: json!({\"message\": \"Internal Server Error\"}),
        status: Status::InternalServerError,
    }
}

pub fn service_unavailable() -> APIResponse {
    APIResponse {
        data: json!({\"message\": \"Service Unavailable\"}),
        status: Status::ServiceUnavailable,
    }
}
";
