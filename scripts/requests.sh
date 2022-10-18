#!/bin/zsh


curl -X POST 'http://localhost:8000/api/v1/auth/token' -H 'Content-Type: application/json' -d '{"id": "a", "permissions": ["OP_GET_SECURED_INFO"]  }'

curl http://localhost:8000/api/v1/demo/manager -H 'Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpZCI6Inl1IiwicGVybWlzc2lvbnMiOlsiUk9MRV9BRE1JTiJdLCJleHAiOjE2NjYxNDY5MTV9.9sNpseVq7u-zgSNI2kbYMW7b_fMF1RlwVMcrGgjk1_0' -H 'Content-Type: application/json'  -v

curl http://localhost:8000/api/v1/demo/admin -H 'Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpZCI6Inl1IiwicGVybWlzc2lvbnMiOlsiUk9MRV9BRE1JTiJdLCJleHAiOjE2NjYxNDY5MTV9.9sNpseVq7u-zgSNI2kbYMW7b_fMF1RlwVMcrGgjk1_0' -H 'Content-Type: application/json'  -v

curl http://localhost:8000/health
