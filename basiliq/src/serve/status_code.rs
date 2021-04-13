use ciboulette::CibouletteResponseStatus;

pub fn convert_status_code(ciboulette_status_code: CibouletteResponseStatus) -> hyper::StatusCode {
    match ciboulette_status_code {
        CibouletteResponseStatus::Ok => hyper::StatusCode::OK,
        CibouletteResponseStatus::OkAsync => hyper::StatusCode::ACCEPTED,
        CibouletteResponseStatus::OkEmpty => hyper::StatusCode::NO_CONTENT,
        CibouletteResponseStatus::Created => hyper::StatusCode::CREATED,
        CibouletteResponseStatus::Unsupported => hyper::StatusCode::BAD_REQUEST,
        CibouletteResponseStatus::Conflict => hyper::StatusCode::CONFLICT,
        CibouletteResponseStatus::NotFound => hyper::StatusCode::NOT_FOUND,
    }
}
