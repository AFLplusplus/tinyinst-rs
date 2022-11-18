#include "aflcov.h"
#include "common.h"
#include "coverage.h"
#include <algorithm>
void AFLCov::GetCoverage(VecCoverage &coverage, bool clear_coverage)
{
    CollectCoverage();
    for (ModuleInfo *module : instrumented_modules)
    {
        ModuleCovData *data = (ModuleCovData *)module->client_data;

        if (data->collected_coverage.empty())
            continue;

        // check if that module is already in the coverage list
        // (if the client calls with non-empty initial coverage)
        ModuleCoverage *module_coverage =
            GetModuleCoverage(coverage, module->module_name);
        if (module_coverage)
        {
            module_coverage->offsets.insert(data->collected_coverage.begin(),
                                            data->collected_coverage.end());
        }
        else
        {
            coverage.push_back({module->module_name, data->collected_coverage});
        }
    }
    if (clear_coverage)
        ClearCoverage();
}

// sets (new) coverage to ignore
void AFLCov::IgnoreCoverage(VecCoverage &coverage)
{
    for (const ModuleCoverage &mod_cov : coverage)
    {
        ModuleInfo *module = GetModuleByName(mod_cov.module_name.c_str());
        if (!module)
            continue;
        ModuleCovData *data = (ModuleCovData *)module->client_data;

        // remember the offsets so they don't get instrumented later
        data->ignore_coverage.insert(mod_cov.offsets.begin(),
                                     mod_cov.offsets.end());
        if (!module->instrumented)
            continue;

        // if we already have instrumentation in place for some of the offsets
        // remove it here
        for (const uint64_t code_off : mod_cov.offsets)
        {
            ClearCoverageInstrumentation(module, code_off);
        }
    }
}

ModuleCoverage *GetModuleCoverage(VecCoverage &coverage, std::string &name)
{
    for (auto iter = coverage.begin(); iter != coverage.end(); iter++)
    {
        if (_stricmp(iter->module_name.c_str(), name.c_str()) == 0)
        {
            return &(*iter);
        }
    }
    return NULL;
}

void MergeCoverage(VecCoverage &coverage, VecCoverage &toAdd)
{
    for (auto iter = toAdd.begin(); iter != toAdd.end(); iter++)
    {
        ModuleCoverage *module_coverage = GetModuleCoverage(coverage, iter->module_name);
        if (module_coverage)
        {
            module_coverage->offsets.insert(
                iter->offsets.begin(),
                iter->offsets.end());
        }
        else
        {
            coverage.push_back({iter->module_name, iter->offsets});
        }
    }
}

void CoverageIntersection(VecCoverage &coverage1,
                          VecCoverage &coverage2,
                          VecCoverage &result)
{
    for (auto iter = coverage1.begin(); iter != coverage1.end(); iter++)
    {
        ModuleCoverage *module1 = &(*iter);
        ModuleCoverage *module2 = GetModuleCoverage(coverage2, iter->module_name);
        if (!module2)
            continue;
        std::set<uint64_t> offsets;
        for (auto offset1 = module1->offsets.begin();
             offset1 != module1->offsets.end(); offset1++)
        {
            if (module2->offsets.find(*offset1) != module2->offsets.end())
            {
                offsets.insert(*offset1);
            }
        }
        if (!offsets.empty())
        {
            result.push_back({iter->module_name, offsets});
        }
    }
}

void CoverageSymmetricDifference(VecCoverage &coverage1,
                                 VecCoverage &coverage2,
                                 VecCoverage &result)
{
    for (auto iter = coverage1.begin(); iter != coverage1.end(); iter++)
    {
        ModuleCoverage *module1 = &(*iter);
        ModuleCoverage *module2 = GetModuleCoverage(coverage2, iter->module_name);
        if (!module2)
        {
            result.push_back({iter->module_name, iter->offsets});
        }
        else
        {
            std::set<uint64_t> offsets;
            for (auto offset1 = module1->offsets.begin();
                 offset1 != module1->offsets.end(); offset1++)
            {
                if (module2->offsets.find(*offset1) == module2->offsets.end())
                {
                    offsets.insert(*offset1);
                }
            }
            for (auto offset2 = module2->offsets.begin();
                 offset2 != module2->offsets.end(); offset2++)
            {
                if (module1->offsets.find(*offset2) == module1->offsets.end())
                {
                    offsets.insert(*offset2);
                }
            }
            if (!offsets.empty())
            {
                result.push_back({iter->module_name, offsets});
            }
        }
    }
    // still need to add coverage for modules in coverage2
    // not present in coverage1
    for (auto iter = coverage2.begin(); iter != coverage2.end(); iter++)
    {
        ModuleCoverage *module2 = &(*iter);
        ModuleCoverage *module1 = GetModuleCoverage(coverage1, iter->module_name);
        if (!module1)
        {
            result.push_back({iter->module_name, iter->offsets});
        }
    }
}

// returns coverage2 not present in coverage1
void CoverageDifference(VecCoverage &coverage1,
                        VecCoverage &coverage2,
                        VecCoverage &result)
{
    for (auto iter = coverage2.begin(); iter != coverage2.end(); iter++)
    {
        ModuleCoverage *module1 = GetModuleCoverage(coverage1, iter->module_name);
        ModuleCoverage *module2 = &(*iter);
        if (!module1)
        {
            result.push_back({iter->module_name, iter->offsets});
        }
        else
        {
            std::set<uint64_t> offsets;
            for (auto offset2 = module2->offsets.begin();
                 offset2 != module2->offsets.end(); offset2++)
            {
                if (module1->offsets.find(*offset2) == module1->offsets.end())
                {
                    offsets.insert(*offset2);
                }
            }
            if (!offsets.empty())
            {
                result.push_back({iter->module_name, offsets});
            }
        }
    }
}

bool CoverageContains(VecCoverage &coverage1, VecCoverage &coverage2)
{
    for (auto iter = coverage2.begin(); iter != coverage2.end(); iter++)
    {
        ModuleCoverage *module2 = &(*iter);
        ModuleCoverage *module1 = GetModuleCoverage(coverage1, iter->module_name);
        if (!module1)
        {
            return false;
        }
        for (auto offset_iter = module2->offsets.begin();
             offset_iter != module2->offsets.end(); offset_iter++)
        {
            if (module1->offsets.find(*offset_iter) == module1->offsets.end())
            {
                return false;
            }
        }
    }
    return true;
}

void WriteCoverage(VecCoverage &coverage, const char *filename)
{
    FILE *fp = fopen(filename, "w");
    if (!fp)
    {
        printf("Error opening %s\n", filename);
        return;
    }
    for (auto iter = coverage.begin(); iter != coverage.end(); iter++)
    {
        for (auto offsetiter = iter->offsets.begin();
             offsetiter != iter->offsets.end(); offsetiter++)
        {
            // skip cmp and other special coverage types
            if (*offsetiter & 0x8000000000000000ULL)
                continue;

            fprintf(fp, "%s+%llx\n", iter->module_name.c_str(), *offsetiter);
        }
    }
    fclose(fp);
}

void WriteCoverageBinary(VecCoverage &coverage, FILE *fp)
{
    uint64_t num_modules = coverage.size();
    fwrite(&num_modules, sizeof(num_modules), 1, fp);
    for (auto iter = coverage.begin(); iter != coverage.end(); iter++)
    {
        uint64_t str_size = iter->module_name.size();
        fwrite(&str_size, sizeof(str_size), 1, fp);
        fwrite(iter->module_name.data(), str_size, 1, fp);
        uint64_t num_offsets = iter->offsets.size();
        fwrite(&num_offsets, sizeof(num_offsets), 1, fp);
        uint64_t *offsets = (uint64_t *)malloc((size_t)num_offsets * sizeof(uint64_t));
        size_t i = 0;
        for (auto iter2 = iter->offsets.begin(); iter2 != iter->offsets.end(); iter2++)
        {
            offsets[i] = *iter2;
            i++;
        }
        fwrite(offsets, sizeof(uint64_t), (size_t)num_offsets, fp);
        free(offsets);
    }
}

void WriteCoverageBinary(VecCoverage &coverage, char *filename)
{
    FILE *fp = fopen(filename, "wb");
    if (!fp)
    {
        printf("Error opening %s\n", filename);
        return;
    }
    WriteCoverageBinary(coverage, fp);
    fclose(fp);
}

void ReadCoverageBinary(VecCoverage &coverage, FILE *fp)
{
    uint64_t num_modules;
    fread(&num_modules, sizeof(num_modules), 1, fp);
    for (size_t m = 0; m < num_modules; m++)
    {
        ModuleCoverage module_coverage;
        uint64_t str_size;
        fread(&str_size, sizeof(str_size), 1, fp);
        char *str_data = (char *)malloc(str_size);
        fread(str_data, str_size, 1, fp);
        module_coverage.module_name = std::string(str_data, str_size);
        free(str_data);
        uint64_t num_offsets;
        fread(&num_offsets, sizeof(num_offsets), 1, fp);
        uint64_t *offsets = (uint64_t *)malloc((size_t)num_offsets * sizeof(uint64_t));
        fread(offsets, sizeof(uint64_t), (size_t)num_offsets, fp);
        for (size_t i = 0; i < num_offsets; i++)
        {
            module_coverage.offsets.insert(offsets[i]);
        }
        free(offsets);
        coverage.push_back(module_coverage);
    }
}

void ReadCoverageBinary(VecCoverage &coverage, char *filename)
{
    FILE *fp = fopen(filename, "rb");
    if (!fp)
    {
        printf("Error opening %s\n", filename);
        return;
    }
    ReadCoverageBinary(coverage, fp);
    fclose(fp);
}

void PrintCoverage(VecCoverage &coverage)
{
    for (auto iter = coverage.begin(); iter != coverage.end(); iter++)
    {
        printf("%s\n", iter->module_name.c_str());
        for (auto offsetiter = iter->offsets.begin();
             offsetiter != iter->offsets.end(); offsetiter++)
        {
            printf("0x%llx ", *offsetiter);
        }
        printf("\n");
    }
}
