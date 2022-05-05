# Project Setup

## Other Links
- [AWS Role Setup](aws_role_setup.md)
- [Cluster Setup](cluster_setup.md)

## Install Required Dependencies
### MACOS
#### Install `kubectl` command line interface (CLI)
```
brew install kubectl
```

#### Install `AWS CLI`
```
curl "https://awscli.amazonaws.com/AWSCLIV2.pkg" -o "AWSCLIV2.pkg"
sudo installer -pkg AWSCLIV2.pkg -target /
```

## Configuring AWS CLI
```
aws configure sso
```

For `sso_start_url` use:
```
https://framework-app.awsapps.com/start/
```

| ***Key***|***Value***|
|-|-|
|`sso_start_url`|`https://framework-app.awsapps.com/start/`|
|`sso_region`|`ap-southeast-2`|
|`cli_default_client_region`|`ap-southeast-2`|
|`output format`|`json`|
|`profile name`|`default`|

<div class="callout"><span class="icon">📖</span>
You will be prompted throughout this process to enter your AWS credentials. if you do not know these, a senior will need to grant you access to your role. If a role was not automatically selected, you will need to be assigned a role by an admin.
</div>

> Your AWS should be fully configured by this point.

---

<style>
    .icon{
        font-size: 22px;
        padding: 4px;
    }
    .callout {
        padding: 16px;
        background: rgba(150, 150, 200, 0.3);
        border-radius: 8px;
        font-weight: 600;
        font-size: 16px;
        display: flex;
        gap: 16px;
        align-items: center;
        margin: 16px 0;
    }

    details {
        padding: 16px;
        border-radius: 8px;
        background: rgba(150, 150, 200, 0.2);
    }

    .callout.red {
        background: rgba(255, 0, 0, 0.15);
    }

    h1 {
        border-bottom: 4px solid #888 !important;
        padding-top: 40px;
    }
</style>
