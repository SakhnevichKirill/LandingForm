struct AvatarTargetFiles {
    // @ApiProperty({ example: `1`, description: `Уникальный идентификатор` })
    id_file: i32,

    // @ApiProperty({ example: `image.png`, description: `Имя картинки` })
    filename: String,

    // @ApiProperty({ example: `binary row`, description: `Бинарная строка` })
    data: Vec<u8>,
}
