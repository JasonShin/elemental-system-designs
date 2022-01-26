<p align="center">
<img src="https://i.imgur.com/9aJnLhB.png" width="55%" />
</p>
<p align="center"><h1 style="text-align: center">Elemental System Designs</h1></p>

<!-- ALL-CONTRIBUTORS-BADGE:START - Do not remove or modify this section -->
[![All Contributors](https://img.shields.io/badge/all_contributors-3-orange.svg?style=flat-square)](#contributors-)
<!-- ALL-CONTRIBUTORS-BADGE:END -->

# What is this?

Elemental System Designs is an open source project to document system architecture design of popular apps and open source projects that we want to study. The project focuses on the high level abstraction architecture based on use cases of an app that we can observe while using them in real life.

We will focus on features with high-impact of the selected apps, while proposing high scalable and resilient architecture that could scale infinitely. We welcome anyone who is actually in the same field as the app of proposed architecture to give us ideas around industry specific bottlenecks and your solution to the problem.

# What is System Design?

System Architecture Design, often called System Design is a conceptual representation of the components and subcomponents that reflects the behaviour of the system as a whole.

System Architects are who make decisions on components to use based on the use cases and requirements given by the business, and consider trade-offs and bottleneck while designing a system.

> What I cannot create, I do not understand. — Richard Feynman

I am writing this project for me to better understand system architecture of various popular apps that I use daily. While the system design proposals are hypothetical, we bring industry best practices in the proposals, and have them available for anyone who is interested in pursuing the field of system architecture designs as references.

> [If you spend enough time on planning, coding is easy.](https://medium.com/the-polyglot-programmer/what-would-sqlite-look-like-if-written-in-rust-part-1-4a84196c217d)

I believe a good architecture is the key to delivering a successful project. A good architecture is required even if you are building a side project or a system for a company while being paid. Once a good architecture is available in front of you well documented, you are more confident and faster as a programmer to hit the deadline of delivering an application (or even a feature within a large ecosystem).

# System Designs

1. [tiktok](./apps/tiktok/README.md)
2. [WIP] Dropbox

# Structure of system designs

### 1. Introduction to the system

This section briefly explains the app. Any useful text from the app's official webiste or Wikipedia would fit for this

### 2. Use cases and Requirements

This section details use cases of the app that we are studying. And followed by requirements for this system design.

When picking requirements, we try to pick the most high-impact features of the app, for example in Tiktok example, we've picked viewing videos, uploading videos, and receiving personalised feeds.

### 3. Constraints

- Identify traffic and data handling at scale
- Scale of the system such as requests per second, request types, data written per second, data read per second
- Special system requirements such as multi-threading

### 4. Drawing Application Architecture Diagram

What is Application Architecture Diagram?

> [An application architecture diagram comprises a high-level overview of the components and fundamental interactions within the system, e.g. microservices, databases, etc. The application architecture diagram primarily addresses the “What” in relation to the system.](https://medium.com/the-internal-startup/how-to-draw-useful-technical-architecture-diagrams-2d20c9fda90d)

We want to keep our system designs to show an abstraction of important components. The diagram should contain following features

- Be simple
- All important components of the system to facilitate the requirements
- System boundaries
  - In an architecture diagram, it is a concept line that divides the system you want to study from 'everything else
  - It is to include elements that you are interested / specified in the use cases / requirements
- Show system interactions
  - Use simple shapes and lines to indicate process flows and the way the different elements communicate to each others
- Include useful annotations
  - Add helpful explanation to critical pieces of your diagram

In the diagram, we want to include following components

- Application service layers
- Data storage layers
- Usually a scalable system includes load balancer, event stream, queue, databases (master/slave, replication or sharding model) and in-memory key-value stores such as Redis

### 5. Component Design

- Component + specific **APIs** required for each of them.
  - The project focuses on the high level design, we should use component design section to enhance the detail that could be too vague in the application architecture diagram. This is the section to write anything important per component level

- Database schema design
  - A simplified database schema design that focused on only important ones in relation to the requirements and use cases

### 6. Understanding Bottlenecks

- Describe any industry specific bottleneck (e.g. security concerns in financial applications)
- Maybe the application needs a CDN? Load balancer? Data is so huge so database should be sharded?
- Maybe the application needs complex query to fetch data, caching the result somewhere such as in-memory cache store would be beneficial?

### 7. Scaling

##### Vertical Scaling

Provide suggestions on adding more power to each component where it's necessary and important.

##### Horizontal Scaling

Wherever possible, service components should be horizontally scalable.

##### Caching suggestions

Can the design take advantage of any following caching strategies?

- Can any component take an advantage of application layer caching [LRU / LFU](https://en.wikipedia.org/wiki/Cache_replacement_policies)?
- Database layer caching?
- In-memory caches such as Redis or Memcache?
- CDN to cache static assets?

# References

- [How to draw 5 types of architectural diagrams](https://www.lucidchart.com/blog/how-to-draw-architectural-diagrams)
- [Introduction to System Architecture Diagram](https://medium.com/backendarmy/introduction-to-system-architecture-design-fcd4f327b6c9)

# Contributing

Please check [CONTRIBUTING.md](https://github.com/JasonShin/elemental-system-designs/blob/master/CONTRIBUTING.md)

## Contributors ✨

Thanks goes to these wonderful people ([emoji key](https://allcontributors.org/docs/en/emoji-key)):

<!-- ALL-CONTRIBUTORS-LIST:START - Do not remove or modify this section -->
<!-- prettier-ignore-start -->
<!-- markdownlint-disable -->
<table>
  <tr>
    <td align="center"><a href="https://github.com/JasonShin"><img src="https://avatars.githubusercontent.com/u/2525002?v=4?s=100" width="100px;" alt=""/><br /><sub><b>Jason Shin</b></sub></a><br /><a href="#infra-JasonShin" title="Infrastructure (Hosting, Build-Tools, etc)">🚇</a> <a href="#ideas-JasonShin" title="Ideas, Planning, & Feedback">🤔</a></td>
    <td align="center"><a href="http://www.daehwa123.com/"><img src="https://avatars.githubusercontent.com/u/11978687?v=4?s=100" width="100px;" alt=""/><br /><sub><b>Tyronne Jeong</b></sub></a><br /><a href="#ideas-TyronneJeong" title="Ideas, Planning, & Feedback">🤔</a></td>
    <td align="center"><a href="https://velog.io/@gyunghoe"><img src="https://avatars.githubusercontent.com/u/41829700?v=4?s=100" width="100px;" alt=""/><br /><sub><b>GummyBearr</b></sub></a><br /><a href="#ideas-GummyBearr" title="Ideas, Planning, & Feedback">🤔</a></td>
  </tr>
</table>

<!-- markdownlint-restore -->
<!-- prettier-ignore-end -->

<!-- ALL-CONTRIBUTORS-LIST:END -->

This project follows the [all-contributors](https://github.com/all-contributors/all-contributors) specification. Contributions of any kind welcome!
