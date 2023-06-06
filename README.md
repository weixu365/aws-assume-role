Assume aws role, save crendentials to profile or expose to environment variables.

Technically, it's the same with executing `aws sts assume-role --role-arn <role-arn> -role-session-name <session-name>`,
and save credential to profile or export to env manually

## Install
- ### Download pre-built executables
    ```bash
    wget https://github.com/weixu365/aws-assume-role/releases/latest/download/aws-assume-role_darwin_amd64 -O aws-assume-role
    chmod +x aws-assume-role
    # Then copy aws-assume-role to one of your PATH folder, e.g. /usr/local/bin
    ```

- ### Install from source
    ```bash
    cargo install --git https://github.com/weixu365/aws-assume-role.git
    ```

## Assume AWS Role
- ### Create/Update profile
    ```bash
    aws-assume-role --profile <profile-name> --session-name <session-name> --role-arn <role-arn>
    ```

    Or add to alias
    ```bash
    alias myrole='aws-assume-role --profile <profile-name> --session-name <session-name> --role-arn <role-arn>'
    ```
    and just execute `myrole` at any time you need

- ### Export into current environment variable using `export`
    ```bash
    eval $(aws-assume-role --session-name <session-name> --role-arn <role-arn>)
    ```
