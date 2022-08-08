# Design

## Kernel
*Kernel, It's fundamentally a hierarchic key-value store*
![Overview](/docs/images/overview.jpg)
## Dependency & lifetime
![Dependency](/docs/images/DependencyStateTime.png)

> Directed Acyclic Graph (DAG)?
> https://hazelcast.com/glossary/directed-acyclic-graph/

> State machine?
> https://doc.rust-lang.org/stable/book/ch17-03-oo-design-patterns.html

## Deployment
![Deployment](/docs/images/IotJobsDeployment.png)
> IoT device SDK?
> https://crates.io/crates/aws-iot-device-sdk
## Configuration
![Configuration](/docs/images/ConfigurationTree.png)
> Hash map?
> https://doc.rust-lang.org/rust-by-example/std/hash.html

> Need concurrent?
> https://github.com/xacrimon/dashmap
## other
![Kernel](/docs/images/KernelTLogInit.png)
