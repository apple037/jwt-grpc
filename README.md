# JWT GRPC implementation
- This is a JWT issue server implementation in grpc, use redis to store validated token
# TODO
- [x] [Generate JWT token](#generate-jet-token)
- [x] [Get token information](#get-token-information)
- [ ] [Revoke token](#revoke-token)
- Redis operation amd AES encryption not done yet
## Generate JWT token
1. Validate request information, here demonstrated with email and password.
2. If token exists issue a new token, if not return the existed token and update expired time instead.
## Get token information
1. Validate request token.
2. Check if token exists, return the claims information.
## Revoke token
1. Validate request token.
2. Check if token exists, once exists delete from redis.