module.exports = {
  preset: "ts-jest",
  testEnvironment: "node",
  testPathIgnorePatterns: [
    "speed.spec.ts",
    "apiv3.spec.ts", // NOTE: Disable backwards compatibility
  ],
};
