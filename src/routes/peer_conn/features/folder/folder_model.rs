use diesel::sql_types::Date;

struct Folder {
    //   @ApiProperty({ example: `1`, description: `Уникальный идентификатор` })
    id_folder: i32,
    //   @ApiProperty({ example: `1`, description: `ID пользователя` })
    name: String,
    // @ForeignKey(() => AvatarFile)
    id_avatar: i32,
}
