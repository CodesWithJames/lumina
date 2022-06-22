# Domain + Load Balancer Records

## Overview
Using AWS's `route 53` service we will setup a domain name for the application.

We will need to create a `Public Hosted DNS Zone` if one does not already exist.

Use the app domain for the zone domain name.

## Zone Setup
Ignore the default automatically inserted records (NS & SOA)

We will create two new records on the `*.example.com` zone. `*` is a wildcard that will match DNS questions to any subdomain of the zone.

When routing traffic, we will need to select an `alias` which AWS will automatically point to our load balancer`

|Record Type|Record Name|Endpoint|
|-----------|----------|---------------------|
|`A` / `IPV4`|`*`|`alias to Network Load Balancer`|
|`AAAA` / `IPV6`|`*`|`alias to Network Load Balancer`|

Select the `region` that the kubernetes cluster was created from (eg: `ap-southeast-2`)

By this point, the domain should be pointing correctly to the cluster.

You may also need to add `A` and `AAAA` records for `example.com` and `www.example.com` to point to the load balancer for the production environment too. This involves leaving the record name field empty.
Repeat the `A` and `AAAA` record and use `*.example.com` for the wildcard, so that the load balancer responds to any subdomain request as well (which we use for testing and development)

## Domain Setup
If you have not already done so, the domain for the application should be configured to have it's nameservers pointing to the nameservers automatically configured within the zone.