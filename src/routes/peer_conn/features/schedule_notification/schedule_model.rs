use diesel::sql_types::Date;

struct ScheduleNotification {
    //   @ApiProperty({ example: `1`, description: `Уникальный идентификатор` })
    id_schedule: i32,
    //   @ApiProperty({ example: `Новый выпуск!`, description: `Уведомление к контенту` })
    notification: String,
    //   @ApiProperty({ example: `1`, description: `ID контент` })
    //   @ForeignKey(() => Content)
    id_content: i32,
    //   @ApiProperty({ example: `30000`, description: `Дата создания токена` })
    timestamp: Date,
}
