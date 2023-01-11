# `ddc-node`

![https://github.com/ThalusA/ddc-node/actions](https://github.com/ThalusA/ddc-node/workflows/CI/badge.svg)

`ddc-node` is a high-level [DDC/CI](https://en.wikipedia.org/wiki/Display_Data_Channel) library for Node written in Rust for efficient and fast control of hardware.

## Usage

```typescript
import { DisplayManager, VCPFeatureCode } from "ddc-node";

const displays = new DisplayManager().collect();

for (const display of displays) {
    const vcp_feature = display.getVcpFeature(VCPFeatureCode.ImageAdjustment.Luminance);
    console.info(`Display at index ${display.index} have a brightness value of`);
    console.info(vcp_feature);
    display.setVcpFeature(VCPFeatures.ImageAdjustment.Luminance,
                          vcp_feature.currentValue + 5);
}
```
