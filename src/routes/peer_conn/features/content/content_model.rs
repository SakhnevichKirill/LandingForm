use diesel::sql_types::Date;

struct Content {
    //   @ApiProperty({ example: `1`, description: `Уникальный идентификатор` })
    id_content: i32,
    //   @ApiProperty({ example: `Контент`, description: `Заголовок к контенту` })
    name: String,
    //   @ApiProperty({ example: `...`, description: `Тэги к контенту` })
    tag: String,
    //   @ApiProperty({ example: `...`, description: `Статистика по контенту` })
    stats: String,
    //   @ApiProperty({ example: `...`, description: `Главное тело контента` })
    json_path: String,
    //   @ApiProperty({ example: `30000`, description: `Дата последнего обновления` })
    timestamp: Date,
}
