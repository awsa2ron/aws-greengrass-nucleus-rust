# aws-greengrass-nucleus-rust
aws greengrass nucleus in Rust (unofficial)


---
> ### What I cannot create, I do not understand.
>
> ### Know how to solve every problem that has been solved. 
>
>                                       ————  Richard Feynman 
>> on his blackboard at the time of death in February 1988; 
>>

---

## Nucleus
The Greengrass nucleus component (aws.greengrass.Nucleus) is a **mandatory** component and the **minimum** requirement to run the AWS IoT Greengrass Core software on a device. 

### a little bit of history

- [v1 preview in 2016/11](https://aws.amazon.com/about-aws/whats-new/2016/11/announcing-aws-greengrass-now-in-limited-preview/)
- [v1 release in 2017/06](https://aws.amazon.com/about-aws/whats-new/2017/06/aws-greengrass-is-now-generally-available/)
- [v2 release in 2020/12](https://www.youtube.com/watch?v=fBNG8OglRZQ)
- [v1 will dead on 2023/06](https://docs.aws.amazon.com/greengrass/v1/developerguide/what-is-gg.html)

### How big?
#### v1 (maybe golang?)
    - Minimum 128 MB disk space available for the AWS IoT Greengrass Core software. If you use the OTA update agent, the minimum is 400 MB.

    - Minimum 128 MB RAM allocated to the AWS IoT Greengrass Core software. With stream manager enabled, the minimum is 198 MB RAM.

#### v2 (open source in Java)
    - memory,
    The maximum amount of RAM (in kilobytes) that each component's processes can use on the core device.

### How v2 works?
**Greengrass v2 CLI**

core-device:
- list-core-devices
- get-core-device
- delete-core-device

component:
- create-component-version
- describe-component
- delete-component
- get-component
- get-component-version-artifact
- list-component-versions
- list-components
- resolve-component-candidates

deployments:
- create-deployment
- list-deployments
- get-deployment
- cancel-deployment

misc:
- list-effective-deployments
- list-installed-components
- list-tags-for-resource
- tag-resource
- untag-resource

## How to

open documentation:
> cargo doc --open

Compile local packages and all of their dependencies:
> cargo b(uild)

Release build:
> cargo b --release

Cross-compile:
> rustup target list
> rustup target add <arch->
> cargo b --target <>

Cross-compile for Raspberry:
> https://robamu.github.io/post/cross-compile-rust-rpi/


Cross-compile for OpenWRT:
> https://blog.dend.ro/building-rust-for-routers/

Fast check:
> cargo c(heck)

Run:
> cargo run (-- <your-args>)

Test:
> cargo test

Documentation tests:
> https://doc.rust-lang.org/rustdoc/write-documentation/documentation-tests.html#documentation-tests

Publish:
> cargo publish

## Design

### Kernel
### *Kernel, It's fundamentally a hierarchic key-value store*
![Overview](/docs/images/Overview.jpg)
### Dependency & lifetime
![Dependency](/docs/images/DependencyStateTime.png)

> Directed Acyclic Graph (DAG)?
> https://hazelcast.com/glossary/directed-acyclic-graph/

> State machine?
> https://doc.rust-lang.org/stable/book/ch17-03-oo-design-patterns.html

### Deployment
![Deployment](/docs/images/IotJobsDeployment.png)
> IoT device SDK?
> https://crates.io/crates/aws-iot-device-sdk
### Configuration
![Configuration](/docs/images/ConfigurationTree.png)
> Hash map?
> https://doc.rust-lang.org/rust-by-example/std/hash.html

> Need concurrent?
> https://github.com/xacrimon/dashmap
### other
![Kernel](/docs/images/KernelTLogInit.png)