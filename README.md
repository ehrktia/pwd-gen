## pwd-gen

![password-generator](asset/src.jpg)

This tool is to improve security and workflow for user management in postgres servers.

In general when a new user is created, the password is either shared via organisation's password manager or as a message in slack(this is true in most case).

### workflow

- SRE/DBA creates a new user in server with only login permission and validity set to expire in few hours (12/24 hours)

- SRE/DBA shares the user name with user, the user who requires to log in and read
  
  data needs to use reset the password

### usage

The app is based on server->client architecture.

**Reason -** The admin user/super user used to manage the password and validity for users are loaded via env vars. The aim of the app is to secure user credentials to data

by following server-> client approach the server controls all env variables and detached from client layer (which is shared with users).

Based on above decision, any user can expand their validity and get password reset 

via cli using below command.

**testing server on standalone**

```bash
⋊> ~/g/pwd-gen on main  grpcurl -plaintext -d '{"user":"test_user"}' \
 --import-path ./protos --proto pwd_gen.proto \
 [::1]:50051 pwdgen.pwdGenerator.UpdatePwdValidity                                   
{
  "user": "test_user",
  "pwd": "2I382ZG8",
  "expiryAt": "2024-04-20 23:20:17.701779945"
}
```

 **testing client using cli**

```bash
⋊> ~/g/pwd-gen on main ⨯ cargo run --bin client -- -u test_user  
 Compiling pwd-gen v0.1.0 (/home/ehrktia/github.com/pwd-gen)
 Finished dev [unoptimized + debuginfo] target(s) in 1.27s
 Running target/debug/client -u test_user
client connecting to:http://[::1]:50051
user:"test_user"
pwd:"7j1CGqfj"
expire_at:"2024-04-21 10:50:09.484403869"
```

### contributing

If you feel like contributing please feel free to raise a PR . Kindly follow [commitlint](https://commitlint.io/) .

### issues

feel free to raise any issues in github issue   

### Todo

- [x] update bin build approach so client can be shared as pre-built binary

- [ ] filter the super users in server layer to avoid accidental password reset via client

- [x] fix client testing via cli and args 

- [ ] add unit test to check logic for invalid user
