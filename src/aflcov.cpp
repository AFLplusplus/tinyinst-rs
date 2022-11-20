#include "aflcov.h"
#include <iostream>
void AFLCov::GetCoverage(Coverage &coverage, rust::Vec<uint64_t> &afl_coverage, bool clear_coverage)
{
    CollectCoverage();
    for (ModuleInfo *module : instrumented_modules)
    {
        ModuleCovData *data = (ModuleCovData *)module->client_data;

        if (data->collected_coverage.empty())
            continue;

        // check if that module is already in the coverage list
        // (if the client calls with non-empty initial coverage)
        std::cout << module->module_name << std::endl;
        ModuleCoverage *module_coverage =
            GetModuleCoverage(coverage, module->module_name);
        if (module_coverage)
        {
            module_coverage->offsets.insert(data->collected_coverage.begin(),
                                            data->collected_coverage.end());

            // Copy the coverage to afl_coverage
            std::copy(data->collected_coverage.begin(), data->collected_coverage.end(), std::back_inserter(afl_coverage));
            // printf("afl_coverage.size() = %d")
        }
        else
        {
            coverage.push_back({module->module_name, data->collected_coverage});
            std::copy(data->collected_coverage.begin(), data->collected_coverage.end(), std::back_inserter(afl_coverage));
        }
    }

    if (clear_coverage)
        ClearCoverage();
}
// void LiteCov::GetCoverage(Coverage &coverage, bool clear_coverage)
// {
//     CollectCoverage();
//     for (ModuleInfo *module : instrumented_modules)
//     {
//         ModuleCovData *data = (ModuleCovData *)module->client_data;

//         if (data->collected_coverage.empty())
//             continue;

//         // check if that module is already in the coverage list
//         // (if the client calls with non-empty initial coverage)
//         ModuleCoverage *module_coverage =
//             GetModuleCoverage(coverage, module->module_name);
//         if (module_coverage)
//         {
//             module_coverage->offsets.insert(data->collected_coverage.begin(),
//                                             data->collected_coverage.end());
//         }
//         else
//         {
//             coverage.push_back({module->module_name, data->collected_coverage});
//         }
//     }
//     if (clear_coverage)
//         ClearCoverage();
// }

// // sets (new) coverage to ignore
// void LiteCov::IgnoreCoverage(Coverage &coverage)
// {
//     for (const ModuleCoverage &mod_cov : coverage)
//     {
//         ModuleInfo *module = GetModuleByName(mod_cov.module_name.c_str());
//         if (!module)
//             continue;
//         ModuleCovData *data = (ModuleCovData *)module->client_data;

//         // remember the offsets so they don't get instrumented later
//         data->ignore_coverage.insert(mod_cov.offsets.begin(),
//                                      mod_cov.offsets.end());
//         if (!module->instrumented)
//             continue;

//         // if we already have instrumentation in place for some of the offsets
//         // remove it here
//         for (const uint64_t code_off : mod_cov.offsets)
//         {
//             ClearCoverageInstrumentation(module, code_off);
//         }
//     }
// }