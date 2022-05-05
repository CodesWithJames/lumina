# Setting up load balancer ingress

## Overview
We use a custom built ingress called `drawbridge-ingress` built by Albert.

It is responsible for proxying incoming http/https traffic to the different pods and services in the cluster.

In other words, our domain on AWS needs to be pointing to the AWS load balancer, which will in turn direct traffic to our ingress `daemonset` pods

### Setting up the AWS Network Load Balancer
Under `EC2` we will find `Load balancers`. We need to create a `Network Load Balancer` for the `drawbridge-ingress`.

It should be `Internet-facing`, and `dualstack` meaning it has both IPV4 and IPV6 addresses.

Select the cluster `VPC` which should've been automatically created during the cluster setup steps.

Select all the `public subnets` in the mappings, and use AWS assigned IPv4 addresses. The subnets should have been automatically setup as well during previous steps. There should be 1 per zone. eg:
- `APSOUTHEAST2A`
- `APSOUTHEAST2B`
- `APSOUTHEAST2C`

### Target Groups
Before we set up the listeners, we will need to create two `target groups`. One for `HTTP Port 80` and one for `HTTPS Port 443`.

For both target groups we will select `instances` as the target type.


|Each Target Group Name|Protocol:Port|Health Check Protocol|
|-----------------|--------|---------------------|
|`yourapp-http-target-group`|`TCP:80`|`HTTP`|
|`yourapp-https-target-group`|`TCP:443`|`HTTP`|

Ensure the `VPC` is the same as the cluster `VPC`

> FOR `HEALTH CHECK PATH` USE `/health-check`
>
> **USE PROTOCOL `HTTP:80` FOR BOTH HTTP AND HTTPS HEALTH CHECKS**
>
> Double check the port override is set to 80


### Target Group Nodes
Select all of the instances in the cluster.

These specific ports are needed because the `drawbridge-ingress` sets up a kubernetes `NodePort` service which redirects incoming traffic from these ports to the `drawbridge-ingress` pods.

|Each Target Group Name|Instance Ports|
|-----------------|--------|
|`yourapp-http-target-group`|`30001`|
|`yourapp-https-target-group`|`30002`|

`Include as pending below` all instances as targets for the target group.

Repeat the process for each target group.

After both target groups are created, we can return back to the load balancer creation, and set up the listeners.

## Listener Setup
For the listeners, add the following:

|Protocol|Port|Target Group|
|--------|----|-----------|
|`TCP`|`80`|`yourapp-http-target-group`|
|`TCP`|`443`|`yourapp-https-target-group`|

Click `Create load balancer`. After this, the load balancer should be ready but there is a few more steps before it can start resolving requests to our application.

## Enable cross-zone load balancing.
Don't forget to enable `Cross-zone load balancing`

## Setting up drawbridge-ingress daemonset

## Setting up security groups
In order to allow the load balancer's incoming traffic to reach the `NodePort` service that the `drawbridge-ingress.yml` creates, we need to create a security group.

We can find the security groups within the AWS `EC2` service.

Look for a security group named something like:
> `eks-cluster-sg-YOURCLUSTERNAME-123456789`

Select the security group, and go edit the `Inbound rules` section.

|Type|Port Range|Source|Description|
|----|---------|-------|-----------|
|`TCP`|`30001 - 30002`|`Anywhere IPv4`|`Allows public access to the drawbridge-ingress NodePort`|
|`TCP`|`30001 - 30002`|`Anywhere IPv6`|`Allows public access to the drawbridge-ingress NodePort`|

Save the rules and continue to the next steps.

## Next Steps
- [Domain Setup](domain-setup)