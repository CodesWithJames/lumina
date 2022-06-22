

## Overview
AWS uses a service called IAM to manage users and permissions, as well as AWS SSO to manage the authentication process.

### Groups

Create a few groups to manage permissions.

Etc:
- engineer
- junior_enginer
- superadmin


## Setting up AWS SSO
- Goal: Set up AWS SSO for authentication via Google GSuite accounts
- How to: Figure it out on google by searching "aws sso gsuite"

# Giving CircleCI Access to the Cluster
### Create IAM User

- First go to IAM on AWS.
- Navigate to users on the side dashboard
- Add User using the username `lumina-circle-ci-user`
- Add `Access Key` as permissions
- Continue the wizard until user is created
- Copy the `User ARN`

Use programmatic API Access key for setting up circle ci automated deployments. Copy the user ARN and paste it in the example below.

### Add circle ci user to cluster auth map
```
kubectl edit -n kube-system configmap/aws-auth
```
**Add something like this**

```
  mapUsers: |
    - userarn: arn:aws:iam::319923562419:user/lumina-circle-ci-user
      username: lumina-circle-ci-user
      groups:
        - circle-ci-role
```

### Create a role for our circle-ci-user to be able to access the cluster
```
kubectl apply -f ./kubernetes/manual/circle-ci-role.yml
```

Next [load-balancer](load-balancer)