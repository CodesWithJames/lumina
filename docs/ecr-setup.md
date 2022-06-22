
## ECR Setup

After cluster has been setup, and the load balancer, you need to create the container image repositories.

In AWS search for `Elastic Container Registry`.

Create the following repositories:
- `yourapp-images/frontend`
- `yourapp-images/backend/`
- `yourapp-images/ingress`

- Visibility should be set to `Private`
- Tag immediately set to `Off`
- Leave all other settings as default

`kubectl apply -f kubernetes/manual/drawbridge-ingress.yml`