# Deployment System


## Overview
- We use `CircleCI` for our `CI/CD` pipeline
  - This works by creating `jobs` for each of our projects
  - `jobs` are a set of steps that are run in order to build and deploy our application
- We use `AWS` for cloud hosting, and have a kubernetes cluster running on AWS for our app
- Each component of the app is built from a `Dockerfile`, and is uploaded to the AWS elastic container registry (`ECR`)
- The jobs include a deploy step, which involve applying kubernetes configurations to the cluster, including changes to updated container images
- `CircleCI` should be configured with secret variables for our AWS credentials needed to deploy to production
- We have a custom built ingress called `drawbridge-ingress` which is reponsible for serving incoming traffic towards our kubernetes pods/containers
- The domain (`*.lumina.earth`) is configured to point to the ingress, who will then
forward the traffic to the kubernetes pods/containers based on the ingress rules


## Components

### Frontend
Our frontend is built using the `svelte` frontend framework, using `svelte-kit` as the routing system.

Within `./frontend`, you will find a `Dockerfile` which outlines the build steps for the frontend.

This Dockerfile is used to generate the container image for the frontend.

### Ingress
We use a custom built ingress called `drawbridge-ingress` which is reponsible for serving incoming traffic towards our kubernetes pods/containers

Inside `/kubernetes` you will find the `production` folder, which will define all the ingress rules for the app.

### Backend
TODO

This backend is built with `Rust`

---

## Misc Setup

1. To modify cluster permissions & settings
```
kubectl edit -n kube-system configmap/aws-auth
```

2. Create a role for our circle-ci-user to be able to access the cluster
kubectl apply -f ./kubernetes/setup/circle-ci-role.yml