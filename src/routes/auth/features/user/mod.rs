mod user_model;
// TODO: implement routs in Controller:
// Put('/users/add-role')
// Delete('/users/del-role')
// Get(/users/settings)
// Post(/users/settings)
// Post('/users/avatar')
// Put('/users/ban')
// Get(/users/all)

// TODO: implement fn in Service:
// createUser(dto: RegisterDto) -> User
// updateEmail(user: User, email: string, t: Transaction) -> number
// verifyUser(id: number, t: Transaction) -> [number]
// getUserById(id: number) -> Option<User>
// getUserByEmail(email: string) -> Option<User>
// getUserByName(name: string) -> Option<User>
// getOnlineUsers(name: string) -> Option<User>
// ban(dto: BanUserDto) -> User
// setPassword(email: string, newPassword: string) -> boolean
// validateUserByName(username: string) -> User
// validateUserById(id: number) -> User
// hasRole(user: User, role: Role) -> boolean
// addRole(dto: RoleDto, t: Transaction) -> boolean
// delRole(dto: RoleDto) -> boolean
// addAvatar(user: User, imageBuffer: Buffer, filename: string) -> AvatarFile
