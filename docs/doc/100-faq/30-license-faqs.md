---
title: Licensing FAQs
---

Databend code is licensed in two ways:

* Apache 2.0 License (Apache)
* Elastic 2.0 License (Elastic)

Databend core is free to use. Most of the core features are licensed under the Apache License, while 
features under `src/query/ee` and `src/meta/ee` directory are subject to the Elastic License.

Databend Enterprise features require a [paid license](#obtain-a-license) from databend and are licensed under the Elastic License.

:::note
You can find the feature's license by taking a look on the code file's header under the [Databend repository](https://github.com/datafuselabs/databend)
:::

## Types of licenses

| Type                | Description                                                                                                                                                                                                                                                                                         |
|---------------------|-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| Apache 2.0 License  | Core features under the Apache License are free to use and fully open source.                                                                                                                                                                                                                       |
| Elastic 2.0 License | * Elastic (Free) features are free to use. The source code is available to view and modify under Elastic 2.0 License Limitations  <br/> * Elastic (Paid) features require an Enterprise License key to access. The source code is available to view and modify under Elastic 2.0 License Limitations | 

For additional custom licensing options, [contact us](https://www.databend.com/contact-us).


## Obtain a license
All Databend code is included in the same binary. No license key is required to access Apache and Elastic (Free) features. To access Elastic (Paid) features, users have two options:
* An **Enterprise license** enables you to use Databend Enterprise features for longer periods (one year or more). To upgrade to Enterprise license, [contact sales](https://www.databend.com/contact-us)
* A **Trial license** enables you to try out Databend for 15 days for free, [contact us](https://www.databend.com/contact-us) to get your trial license.

:::note
Databend Labs encourage non-commercial academic research involving Databend. For such projects, please [contact us](https://www.databend.com/contact-us) for possible long term licenses)
:::

## FAQs

### Can I host Databend as a Service for internal use at my organization?
Yes, employees and contractors can use your internal Databend instance as a service under the Elastic license since it was created. 
Use of Enterprise features will always require a license.

### Why databend choose Elastic License 2.0 for enterprise features?
the Elastic License 2.0 provides a good balance between open-source values and commercial interests.
Comparing other license such as Business Source License, Custom Community License, Elastic License 2.0 is simple, short and clear.
There only have three limitations applied:
1. Cannot provide software as a hosted or managed service with substantial access to features/functionality.
2. Cannot modify or circumvent license key functionality or remove/obscure protected functionality.
3. Cannot alter/remove/licensing, copyright, or trademark notices of the licensor in the software.


### I would like to reuse some components from the Databend project in my own software, which uses the AGPL or another open source license, Is this possible?
The Databend team is committed to supporting the open-source community and willing to consider extracting specific internal components that are generally useful as a separate project with its own license, for example APL.
For more details, feel free to [contact us](https://www.databend.com/contact-us).

### Can you provide some examples around what qualifies as "providing the software to third parties as a hosted or managed Service" or not?

**I'm using databend for data dashboard on my analytic SaaS product**

This is permitted under ELv2.

**I'm an analytic engineer setting up Databend for my organization to use internally**

This is permitted under ELv2, because you are not providing the software as a managed service.

**I am a Managed Service Provider running Databend for my customers**

If your customers do not access Databend. this is permitted under ELv2. 
if your customers do have access to substantial portions of functionality of Databend as part of your service, this may not be permitted.

If you have questions about your scenarios or want more flexible license options, please [contact us](https://www.databend.com/contact-us)
