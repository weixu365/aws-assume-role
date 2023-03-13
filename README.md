Assume aws role, save crendentials to profile or expose to environment variables.

Technically, it's the same with executing `aws sts assume-role --role-arn <role-arn> -role-session-name <session-name>`,
and save credential to profile or export to env manually

## Create/Update profile
aws-assume-role --profile <profile-name> --session-name <session-name> --role-arn <role-arn>

## Export into current environment variable using `export`
eval $(aws-assume-role --session-name <session-name> --role-arn <role-arn>)
