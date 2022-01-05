

# Deployment Process


install AWS CLI

Windows: https://awscli.amazonaws.com/AWSCLIV2.msi
MacOS: https://awscli.amazonaws.com/AWSCLIV2.pkg

## Configuring AWS CLI
```
aws configure
```

-> get access key id from IAM on management console
-> get secret access key on management console
-> default region name will be ap-southeast-2


```
aws eks update-cluster-config --region ap-southeast-2 --name lumina
aws eks update-cluster-config \
    --region ap-southeast-2 \
    --name lumina \
    --resources-vpc-config endpointPublicAccess=<true>,publicAccessCidrs="<203.0.113.5/32>",endpointPrivateAccess=<true>
```

## Connecting to the cluster

```
aws eks --region ap-southeast-2 update-kubeconfig --name lumina
```


## Deploying

```
kubectl apply -f ./kubernetes/deployment/frontend-deployment.yml
```

kubectl apply -f ./kubernetes/deployment/eks-admin-service-account.yml


```
aws cloudformation create-stack --stack-name lumina-vpc-stack --template-url https://s3.us-west-2.amazonaws.com/amazon-eks/cloudformation/2020-10-29/amazon-eks-vpc-private-subnets.yaml

aws iam create-role --role-name lumina-eks-role --assume-role-policy-document file://"C:\repos\lumina\cluster-role-trust-policy.json"

aws iam attach-role-policy --policy-arn arn:aws:iam::aws:policy/AmazonEKSClusterPolicy --role-name lumina-eks-role

```

SET AWS_ACCESS_KEY_ID=
SET AWS_SECRET_ACCESS_KEY=
SET AWS_SESSION_TOKEN=

# Local Dev

kubectl apply -f https://raw.githubusercontent.com/kubernetes/ingress-nginx/controller-v0.41.2/deploy/static/provider/cloud/deploy.yaml


devspace dev