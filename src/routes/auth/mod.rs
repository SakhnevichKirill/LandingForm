pub mod auth_controller;
pub mod auth_dto;
pub mod auth_service;
pub mod features;

use axum::Router;

use self::auth_controller::AuthController;

use super::AppState;

/// This function returns a router with routes
/// for authentication.
pub fn init_auth_router(app_state: AppState) -> Router<AppState> {
    Router::new().nest(
        "/auth",
        AuthController::init_auth_router().with_state(app_state.clone()),
    )
} // end fn get_auth_routes

// TODO: implement routs in Controller:
// @Post(`/registration`)
// @Post(`/login`)
// @Post('/refresh-token')
// @Get('/forgot-password/:email')
// @Post('/reset-password')

// TODO: implement fn in Service:
// login(user: User) -> TokenResponse
// registration(userDto: RegisterDto) -> TokenResponse
// loginWithRefreshToken(refreshToken: string) -> TokenResponse
// logoutFromAllDevices(user: User)
// sendEmailForgotPassword(email: string) -> IResponse
// validatePassword(email: string, password: string) -> boolean
// validate(username: string, password: string) -> User
// setNewPassword(resetPasswordDto: ResetPasswordDto) -> IResponse

// getRefreshTokenOptions(user: User) -> JwtSignOptions
// getAccessTokenOptions(user: User) -> JwtSignOptions
// getTokenOptions(type: 'REFRESH' | 'ACCESS', user: User | null) -> JwtSignOptions

// loginWithThirdParty(fieldId: keyof User) -> TokenResponse // for example auth with Google
