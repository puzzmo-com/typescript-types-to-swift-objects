# TypeScript type -> Swift Object CLI 

This project aims to find TypeScript type definitions like:

```ts
type AppSpecificPartner = {
  backURL: string
  navHeight: number
  logoLongBlack: string
  logoLongWhite: string
  logoHeight: number
  logoWidth: number
  ourLogoOffsets: readonly number[] // [number, number]
  theirLogoOffsets: readonly number[] //[number, number]
  appearance: PublishingPartnerAppearance
  navBG: PublishingPartnerNavBackground
  navFG: PublishingPartnerNavForeground
  ourLogoFG: PublishingPartnerNavBackground
  ourLogoFGOverride: string | null | undefined
}

type MessagesSent =
  | {
      type: "app-context"
      userStateID: string
      userID: string | undefined
      partnerID: string | undefined
      partner: AppSpecificPartner | null | undefined
      theme: Theme
    }
  | {
      type: "start-sub"
      successURL: string
      failURL: string
      accountSlug: string
      partnerSlug: string
      impactClickID: string
    }
  | {
      type: "start-gift"
      successURL: string
      failURL: string
      accountSlug: string
      partnerSlug: string
      impactClickID: string
    }

```

From inside a .ts file, and convert it to a Swift Object like:

```swift
struct AppSpecificPartner: Codable {
    let backURL: String
    let navHeight: Int
    let logoLongBlack: String
    let logoLongWhite: String
    let logoHeight: Int
    let logoWidth: Int
    let ourLogoOffsets: [Int]
    let theirLogoOffsets: [Int]
    let appearance: PublishingPartnerAppearance
    let navBG: PublishingPartnerNavBackground
    let navFG: PublishingPartnerNavForeground
    let ourLogoFG: PublishingPartnerNavBackground
    let ourLogoFGOverride: String?
}

enum MessagesSent {
    case appContext(MessagesSentAppContext)
    case startSub(MessagesSentStartSub)
    case startGift(MessagesSentStartGift)
}

struct MessagesSentAppContext: Codable {
    let type: String
    let userStateID: String
    let userID: String?
    let partnerID: String?
    let partner: AppSpecificPartner?
    let theme: Theme
}

struct MessagesSentStartSub: Codable {
    let type: String
    let successURL: String
    let failURL: String
    let accountSlug: String
    let partnerSlug: String
    let impactClickID: String
}

struct MessagesSentStartGift: Codable {
    let type: String
    let successURL: String
    let failURL: String
    let accountSlug: String
    let partnerSlug: String
    let impactClickID: String
}



```

OOTB the tool will be a CLI to generate this, but perhaps the end goal will also be to make a WASM build.


## How to work on it

```bash
git clone https://github.com/puzzmo-com/typescript-types-to-swift-objects.git
cd typescript-types-to-swift-objects

# Install dependencies
cargo build

# Run an example
cargo run -- test/example.ts
```
