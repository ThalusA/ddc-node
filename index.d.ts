/* tslint:disable */
/* eslint-disable */

/* auto-generated by NAPI-RS */

export const enum VcpValueType {
  Continuous = 0,
  NonContinuous = 1,
  Table = 2,
}
export interface Continuous {
  currentValue: number
  maximumValue: number
  type: VcpValueType.Continuous
}
export interface NonContinuous {
  currentValue: number
  currentValueRepresentation?: string
  possibleValues: Record<string, string | undefined | null>
  type: VcpValueType.NonContinuous
}
export interface Table {
  currentData: Array<number>
  type: VcpValueType.Table
}
export const enum JsQueryType {
  Backend = 0,
  Id = 1,
  ManufacturerId = 2,
  ModelName = 3,
  SerialNumber = 4,
}
export interface Query {
  queryType: JsQueryType
  queryValue: string
}
export type JsDisplay = Display
export class Display {
  index: number
  backend: string
  edidData?: Uint8Array
  version?: string
  mccsVersion?: string
  displayId: string
  serial?: number
  serialNumber?: string
  modelId?: number
  modelName?: string
  manufacturerId?: string
  manufactureYear?: number
  manufactureWeek?: number
  capabilities?: string
  constructor(index: number)
  getVcpFeature(featureCode: number): Promise<Continuous | NonContinuous | Table>
  setVcpFeature(
    featureCode: number,
    valueOrOffset: number,
    bytes?: Array<number> | undefined | null,
  ): Promise<undefined>
  updateCapabilities(): string | null
}
export type JsDisplayManager = DisplayManager
export class DisplayManager {
  constructor(queries?: Query | Array<Query> | undefined | null)
  static getByIndex(index: number): Display
  get queries(): Array<Query>
  set queries(queries: Array<Query>)
  addQueries(queries?: Query | Array<Query> | undefined | null): void
  collect(): Promise<Array<Display>>
  list(): Promise<Array<Display>>
}
export namespace VCPFeatureCode {
  export const enum PresetFunctions {
    CodePage = 0,
    RestoreFactoryColorDefaults = 8,
    RestoreFactoryDefaults = 4,
    RestoreFactoryGeometryDefaults = 6,
    RestoreFactoryLuminanceContrastDefaults = 5,
    RestoreFactoryTvDefaults = 10,
    SaveRestoreSettings = 176,
  }
  export const enum ImageAdjustment {
    SixAxisHueControlBlue = 159,
    SixAxisHueControlCyan = 158,
    SixAxisHueControlGreen = 157,
    SixAxisHueControlMagenta = 160,
    SixAxisHueControlRed = 155,
    SixAxisHueControlYellow = 156,
    SixAxisSaturationControlBlue = 93,
    SixAxisSaturationControlCyan = 92,
    SixAxisSaturationControlGreen = 91,
    SixAxisSaturationControlMagenta = 94,
    SixAxisSaturationControlRed = 89,
    SixAxisSaturationControlYellow = 90,
    AdjustZoom = 124,
    AutoColorSetup = 31,
    AutoSetup = 30,
    AutoSetupOnOff = 162,
    BacklightControl = 19,
    BacklightLevelBlue = 113,
    BacklightLevelGreen = 111,
    BacklightLevelRed = 109,
    BacklightLevelWhite = 107,
    BlockLutOperation = 117,
    Clock = 14,
    ClockPhase = 62,
    ColorSaturation = 138,
    ColorTemperatureIncrement = 11,
    ColorTemperatureRequest = 12,
    Contrast = 18,
    DisplayApplication = 220,
    FleshToneEnhancement = 17,
    Focus = 28,
    Gamma = 114,
    GrayScaleExpansion = 46,
    HorizontalMoire = 86,
    Hue = 144,
    Luminance = 16,
    LutSize = 115,
    ScreenOrientation = 170,
    SelectColorPreset = 20,
    Sharpness = 135,
    SinglePointLutOperation = 116,
    StereoVideoMode = 212,
    TvBlackLevelLuminance = 146,
    TvContrast = 142,
    TvSharpness = 140,
    UserColorVisionCompensation = 23,
    VelocityScanModulation = 136,
    VerticalMoire = 88,
    VideoBlackLevelBlue = 112,
    VideoBlackLevelGreen = 110,
    VideoBlackLevelRed = 108,
    VideoGainBlue = 26,
    VideoGainGreen = 24,
    VideoGainRed = 22,
    WindowBackground = 154,
    WindowControlOnOff = 164,
    WindowSelect = 165,
    WindowSize = 166,
    WindowTransparency = 167,
  }
  export const enum DisplayControl {
    DisplayControllerId = 200,
    DisplayFirmwareLevel = 201,
    DisplayUsageTime = 198,
    HorizontalFrequency = 172,
    ImageMode = 219,
    OsdButtonLevelControl = 202,
    OsdLanguage = 204,
    PowerMode = 214,
    SourceColorCoding = 181,
    SourceTimingMode = 180,
    Version = 223,
    VerticalFrequency = 174,
  }
  export const enum Geometry {
    BottomCornerFlare = 74,
    BottomCornerHook = 76,
    DisplayScaling = 134,
    HorizontalConvergenceMG = 41,
    HorizontalConvergenceRB = 40,
    HorizontalKeystone = 66,
    HorizontalLinearity = 42,
    HorizontalLinearityBalance = 44,
    HorizontalMirror = 130,
    HorizontalParallelogram = 64,
    HorizontalPincushion = 36,
    HorizontalPincushionBalance = 38,
    HorizontalPosition = 32,
    HorizontalSize = 34,
    Rotation = 68,
    ScanMode = 218,
    TopCornerFlare = 70,
    TopCornerHook = 72,
    VerticalConvergenceMG = 57,
    VerticalConvergenceRB = 56,
    VerticalKeystone = 67,
    VerticalLinearity = 58,
    VerticalLinearityBalance = 60,
    VerticalMirror = 132,
    VerticalParallelogram = 65,
    VerticalPincushion = 52,
    VerticalPincushionBalance = 54,
    VerticalPosition = 48,
    VerticalSize = 50,
    WindowPositionBrX = 151,
    WindowPositionBrY = 152,
    WindowPositionTlX = 149,
    WindowPositionTlY = 150,
  }
  export const enum Miscellaneous {
    ActiveControl = 82,
    AmbientLightSensor = 102,
    ApplicationEnableKey = 198,
    AssetTag = 210,
    AuxiliaryDisplayData = 207,
    AuxiliaryDisplaySize = 206,
    AuxiliaryPowerOutput = 215,
    Degauss = 1,
    DisplayDescriptorLength = 194,
    DisplayIdentificationDataOperation = 135,
    DisplayTechnologyType = 182,
    EnableDisplayOfDisplayDescriptor = 196,
    FlatPanelSubPixelLayout = 178,
    InputSource = 96,
    NewControlValue = 2,
    OutputSelect = 208,
    PerformancePreservation = 84,
    RemoteProcedureCall = 118,
    ScratchPad = 222,
    SoftControls = 3,
    StatusIndicators = 205,
    TransmitDisplayDescriptor = 195,
    TvChannelUpDown = 139,
  }
  export const enum Audio {
    BalanceLR = 147,
    Bass = 145,
    JackConnectionStatus = 101,
    MicrophoneVolume = 100,
    Mute = 141,
    ProcessorMode = 148,
    SpeakerSelect = 99,
    SpeakerVolume = 98,
    Treble = 143,
  }
  export const enum Dpvl {
    BodyCrcErrorCount = 188,
    ClientId = 189,
    HeaderErrorCount = 187,
    LinkControl = 190,
    MonitorStatus = 183,
    MonitorXOrigin = 185,
    MonitorYOrigin = 186,
    PacketCount = 184,
  }
}
