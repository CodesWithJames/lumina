
# Setting up kubernetes cluster

### Installing kubectl
```
curl --silent --location "https://github.com/weaveworks/eksctl/releases/latest/download/eksctl_$(uname -s)_arm64.tar.gz" | tar xz -C /tmp
sudo mv /tmp/eksctl /usr/local/bin
```

### AWS Configure Default settings
`nano ~/.aws/credentials`
```
[default]
sso_start_url = https://framework-app.awsapps.com/start/
sso_region = ap-southeast-2
sso_account_id = 319923562419
sso_role_name = AdministratorAccess
region = ap-southeast-2
output = json
```

### Logging in
1. Call `aws sso login`

### Creating a cluster

```
eksctl create cluster \
    --name lumina-main \
    --region ap-southeast-2 \
    --nodegroup-name lumina-nodes \
    --node-type t2.large \
    --nodes 1
```

### Setting up kubectl to use cluster (not usually required)
```
kubectl config get-contexts
kubectl config use-context albert@insanemarketing.com.au@lumina-main.ap-southeast-2.eksctl.io
```

## Configure AWS ECR image repository
```
aws ecr get-login-password | docker login --username AWS --password-stdin 239930134035.dkr.ecr.ap-southeast-2.amazonaws.com
```

## Setting up AWS NGINX Ingress Controller
Guide: https://itnext.io/save-on-your-aws-bill-with-kubernetes-ingress-148214a79dcb
```
kubectl apply -f https://raw.githubusercontent.com/kubernetes/ingress-nginx/controller-v1.0.4/deploy/static/provider/aws/deploy.yaml
```

## SSL
TODO
Guide
https://www.digitalocean.com/community/tutorials/how-to-set-up-an-nginx-ingress-with-cert-manager-on-digitalocean-kubernetes