# Lumina Government Website & System
## First-time Setup
### First-time setup
- Install Docker on your operating system
- Install VSCode
- Install Git

### Installing repository
In the directory you would like to install the repository, enter the following command.

```
git clone https://github.com/lumina-gov/lumina.git
```

Authenticate with your github account as necessary.

## Setup

### Install devspace globally

**Note:** You will need to install the Node.JS for this command to work

```
sudo npm i -g devspace
```

Or on MacOS:

```
curl -s -L "https://github.com/loft-sh/devspace/releases/latest" | sed -nE 's!.*"([^"]*devspace-darwin-arm64)".*!https://github.com\1!p' | xargs -n 1 curl -L -o devspace && chmod +x devspace;
sudo install devspace /usr/local/bin;
```

### Finish setup
```
sudo devspace
```

### Install homebrew & kubectl

```
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
brew install kubernetes-cli
kubectl version --client
```

### Installing AWS CLI

https://awscli.amazonaws.com/AWSCLIV2.pkg


### AWS Configure Default settings
```
mkdir ~/.aws
nano ~/.aws/credentials
```
Write the following text into the nano text editor, and when finished, press control + x to save the file. (after this, press Y to save, then overwrite the file in the same path)
```
[default]
sso_start_url = https://framework-app.awsapps.com/start
sso_region = ap-southeast-2
sso_account_id = {YOUR USER ACCOUNT ID HERE}
sso_role_name = AdministratorAccess
region = ap-southeast-2
output = json
```

### Install Docker

Install docker


### Connect to kubectl to EKS cluster
```
aws eks --region ap-southeast-2 update-kubeconfig --name lumina-cluster
```

### Set devspace namespace
Use your name where "NAME" is. This will be your personal kubernetes namespace

```
devspace use namespace NAME
```

### Finally
Run the app

```
devspace dev
```

---
## --- END OF SETUP --
---

### Add User to AWS EKS Cluster
https://docs.aws.amazon.com/eks/latest/userguide/add-user-role.html
https://www.powerupcloud.com/aws-eks-authentication-and-authorization-using-aws-single-signon/#:%7E:text=ensure%20to%20remove,role_arn

1. Set vscode as the default editor `export KUBE_EDITOR='code --wait'`
2. Open the aws-auth ConfigMap
```
kubectl edit -n kube-system configmap/aws-auth
```