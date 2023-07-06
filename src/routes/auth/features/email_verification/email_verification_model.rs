use diesel::sql_types::Date;

struct EmailVerification {
    //   @ApiProperty({ example: `1`, description: `Уникальный идентификатор` })
    id_verify: i32,
    //   @ApiProperty({ example: `1`, description: `ID пользователя` })
    //   @ForeignKey(() => User)
    id_user: i32,
    //   @ApiProperty({ example: `12345678`, description: `Сгенерированный токен` })
    email_token: String,
    //   @ApiProperty({ example: `30000`, description: `Дата создания токена` })
    timestamp: Date,
}
