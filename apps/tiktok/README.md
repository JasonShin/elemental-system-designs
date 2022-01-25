<img src="https://i.imgur.com/JMhYYtR.png" width="50%" />

# tiktok

TikTok is a video-focused social networking service owned by Chinese company ByteDance Ltd. It hosts a variety of short-form user videos, from genres like pranks, stunts, tricks, jokes, dance, and entertainment with durations from 15 seconds to three minutes. TikTok is an international version of Douyin, which was originally released in the Chinese market in September 2016. TikTok was launched in 2017 for iOS and Android in most markets outside of mainland China; however, it became available worldwide only after merging with another Chinese social media service, Musical.ly, on 2 August 2018.

# Requirements

1. Watch short videos
2. Upload videos
3. Receive suggested video feeds

# Use cases

#### 1. Watch short videos

- Short videos are downloaded as MP4 format
- The video sizes varies but generally stays under 5mb in size
- The videos are cached in CDN and served by CDN too

#### 2. Upload videos

- Users can record video on their mobile devices for the uploads
  - User uploaded videos can be maximum 500 MB in sizes
  - Once uploaded, we will have a service to compress the video

#### 3. Receive suggested videos

- Every day, there's pre-computed list of suggested video feeds that users receive on next day
- We use Machine Learning to give best matching videos per user that they might be interested

# Constraints

1. Tiktok receives 1 billion active users per month
   1. System should be scalable for even bigger user base in the future
2. On Average, U.S. Adult users spent 33 minutes per day on Tiktok
   1. 24/7 uptime is critical in the design, we need to think about self-recovery of each component in the design. How to recover failure operation through retries or other mechanism

# Application Architecture Diagram

<img src="./assets/tiktok.png" />

# Design Core Components

Use case: user enters the app and gets the initial video feeds

- The **User** sends a request to get the latest video feeds from **Core Service**, running as [reverse proxy](https://en.wikipedia.org/wiki/Reverse_proxy) in front of a [load balancer](https://en.wikipedia.org/wiki/Load_balancing_(computing)).
- **Core Service** queries the fresh user video feeds from **Meta DB**
- 

#### 2. UploadVideoService

...

#### 3. UserActivityObserver

...

#### 4. VideoCacher

...

#### 5. VideoCompressor

...

#### 6. Glacier to Standard

...

#### 7. Standard to Glacier

...



