struct User {
    // @ApiProperty({ example: `1`, description: `Уникальный идентификатор` })
    id: i32,

    // @ApiProperty({ example: `Kir`, description: `Имя пользователя` })
    name: String,

    // @ApiProperty({ example: `user@mail.ru`, description: `Почтовый адрес` })
    email: Option<String>,

    // @ApiProperty({ example: `12345678`, description: `Пароль` })
    password: Option<String>,

    // @ApiProperty({ example: `7`, description: `Код страны` })
    phone_number_code: i32,

    // @ApiProperty({ example: `9999999999`, description: `Номер телефона` })
    phone_number: String,

    // @ApiProperty({
    //   example: `true`,
    //   description: `Статус почты: подтверждена или нет`,
    // })
    verified: bool,

    // @ApiProperty({ example: `13543654`, description: `Токен для сессии` })
    session_token: Option<String>,

    // @ApiProperty({
    //   example: `IDFacebook`,
    //   description: `Регистрационный IDFacebook`,
    // })
    id_facebook: Option<String>,

    // @ApiProperty({ example: `IDGoogle`, description: `Регистрационный IDGoogle` })
    id_google: Option<String>,

    // @ApiProperty({ example: `IDApple`, description: `Регистрационный IDApple` })
    id_apple: Option<String>,

    // @ApiProperty({ example: `true`, description: `Забанен или нет` })
    banned: Option<bool>,

    // @ApiProperty({ example: `За хулиганство`, description: `Причина блокировки` })
    ban_reason: Option<String>,

    // @ForeignKey(() => AvatarFile)
    id_avatar: i32,
}
