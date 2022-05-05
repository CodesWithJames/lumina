# Cluster Setup

<div class="callout red"><span class="icon">🚨</span>
The cluster has likely already been set up for this project, so ignore this. It's only here for documentation purposes.
</div>

## Setting up kubernetes cluster

### Installing kubectl (MacOS)
```
brew install kubectl
```

### Logging in
Call `aws sso login`

### Creating a cluster

```
eksctl create cluster \
    --name lumina-cluster \
    --region ap-southeast-2 \
    --nodegroup-name lumina-nodes \
    --node-type t2.large \
    --nodes 2
```
### Create the production namespace
```
kubectl apply -f ./kubernetes/manual/namespace.yml
```

<details>
<summary><b>Setting up kubectl context (not usually required)</b></summary>
<blockquote>This step should've already been completeted and you should skip it.</blockquote>

To get a list of available contexts, run the following command:
```
kubectl config get-contexts
```
Select a context (name) that makes sense, and run the following command with the selected context:
```
kubectl config use-context CONTEXT_NAME_HERE
```
</details>

## Next steps

- [AWS role setup](aws-role-setup)
- [Setting up load balancer ingress](drawbridge-ingress)


---

<style type="text/css">
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
        border-bottom: 2px solid #888 !important;
        padding-top: 40px;
    }
</style>