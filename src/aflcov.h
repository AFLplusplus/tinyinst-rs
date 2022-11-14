#pragma once
#include "litecov.h"
// #include <unordered_map>
// class AFLCov : public LiteCov {
//  public:
//   AFLCov(uint8_t *_coverage, size_t _capacity) : LiteCov() {
//     coverage = _coverage;
//     capacity = _capacity;
//     cur_map_offset = 0;
//   }

//   void Init(int argc, char **argv) override {
//     LiteCov::Init(argc, argv);
//   }
//   void add_coverage(uint8_t addr) {
//     if (addr < capacity) { coverage[addr] = 1; }
//   }
//   void print_coverage() {
//     for (size_t i = 0; i < capacity; i++) {
//       if (coverage[i] == 1) { printf("Address %llx is covered\n", i); }
//     }
//   }
//   void GetCoverage(Coverage &coverage, bool clear_coverage) {
//     CollectCoverage();
//     for (ModuleInfo *module : instrumented_modules) {
//       ModuleCovData *data = (ModuleCovData *)module->client_data;

//       if (data->collected_coverage.empty()) continue;

//       // check if that module is already in the coverage list
//       // (if the client calls with non-empty initial coverage)
//       ModuleCoverage *module_coverage =
//           GetModuleCoverage(coverage, module->module_name);
//       if (module_coverage) {
//         module_coverage->offsets.insert(data->collected_coverage.begin(),
//                                         data->collected_coverage.end());
//       } else {
//         coverage.push_back({module->module_name, data->collected_coverage});
//       }
//     }
//     if (clear_coverage) ClearCoverage();
//   }

//   uint8_t *coverage;
//   size_t   capacity;
//   size_t   cur_map_offset;
//   // std::unordered_map<uint64_t, uint64_t> addr_offset_map;
// };
