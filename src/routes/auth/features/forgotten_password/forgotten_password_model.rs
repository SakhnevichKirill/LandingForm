use diesel::sql_types::Date;

struct ForgottenPassword {
    //   @ApiProperty({ example: `1`, description: `Уникальный идентификатор` })
    id_recovery: i32,
    //   @ApiProperty({ example: `1`, description: `ID пользователя` })
    //   @ForeignKey(() => User)
    id_user: i32,
    //   @ApiProperty({ example: `12345678`, description: `Сгенерированный токен` })
    new_password_token: String,
    //   @ApiProperty({ example: `30000`, description: `Дата создания токена` })
    timestamp: Date,
}
