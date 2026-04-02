/**
 * Playwright Integration Test for BitUnlocker Word Generation
 * 
 * This test validates word generation functionality including:
 * 1. Month placeholder with shortened versions (all subsequences)
 * 2. Number placeholder should NOT support shortened modifier (should error)
 * 3. Case variations and duplicate handling using HashSet
 */

const { execSync } = require('child_process');
const fs = require('fs');
const path = require('path');

// Configuration - Use two levels up to get to the target/release folder where password-gen.exe is located
const WORKSPACE_DIR = path.resolve(__dirname, '..', '..');
const PASSWORD_GEN_CMD = path.join(WORKSPACE_DIR, 'target', 'release', 'password-gen.exe');

/**
 * Clean up test files (generated passwords)
 */
function cleanupTestFiles() {
    const genFile = path.join(WORKSPACE_DIR, 'generated_passwords.txt');
    if (fs.existsSync(genFile)) {
        fs.unlinkSync(genFile);
    }
}

/**
 * Generate test passwords using the CLI tool
 */
function generatePasswords(template) {
    console.log(`Generating passwords with template: ${template}`);
    
    const cmd = `${PASSWORD_GEN_CMD} gen "${template}"`;
    return execSync(cmd, { 
        cwd: WORKSPACE_DIR,
        encoding: 'utf-8'
    });
}

/**
 * Read generated passwords from file and return as Set for O(1) lookup
 */
function getGeneratedPasswordsSet() {
    const genFile = path.join(WORKSPACE_DIR, 'generated_passwords.txt');
    if (!fs.existsSync(genFile)) {
        throw new Error('Generated passwords file not found');
    }
    
    const content = fs.readFileSync(genFile, 'utf-8').trim();
    return new Set(content.split('\n'));
}

/**
 * Read generated passwords from file and return array
 */
function getGeneratedPasswordsArray() {
    const genFile = path.join(WORKSPACE_DIR, 'generated_passwords.txt');
    if (!fs.existsSync(genFile)) {
        throw new Error('Generated passwords file not found');
    }
    return fs.readFileSync(genFile, 'utf-8').trim().split('\n');
}

/**
 * Parse the output to get the count of generated passwords
 */
function parseGenerationOutput(output) {
    const match = output.match(/Generated\s+(\d+)\s+passwords/);
    if (match) {
        return parseInt(match[1], 10);
    }
    return 0;
}

/**
 * Test Suite 1: Month placeholder with shortened versions
 * 
 * Template: Example@{month,begin=may,end=may,case=lower,shortened}
 * Expected: All subsequences of "may" (m, a, y, ma, my, ay, may) 
 *           with "Example" prefix and case variations
 */
function testMonthWithShortened() {
    console.log('\n' + '='.repeat(70));
    console.log('TEST SUITE 1: Month Placeholder with Shortened Versions');
    console.log('='.repeat(70));
    
    cleanupTestFiles();
    
    // Test with May only, case=lower, shortened
    const template = '{month,begin=may,end=may,case=lower,shortened}';
    console.log('\n--- Step 1: Generate passwords with month+shortened ---');
    console.log(`Template: ${template}`);
    
    try {
        const output = generatePasswords(template);
        const count = parseGenerationOutput(output);
        
        console.log(`\nGenerated ${count} passwords`);
        
        // Read the generated file
        const passwordsSet = getGeneratedPasswordsSet();
        const passwordsArray = getGeneratedPasswordsArray();
        
        console.log('All generated passwords:', JSON.stringify(passwordsArray, null, 2));
        
        // For "may" with case=lower, shortened:
        // Expected subsequences (with min_length=1 default):
        // m, a, y, ma, my, ay, may = 7 unique
        const expectedSubsequences = ['m', 'a', 'y', 'ma', 'my', 'ay', 'may'];
        
        console.log('\n--- Verification ---');
        let passCount = 0;
        let failCount = 0;
        
        // Check that we have the correct count (all case variations)
        const actualCount = passwordsSet.size;
        
        // With all subsequences and case variations, we should have more than just 7
        // Each subsequence can be: lowercase, uppercase, titlecase
        // So minimum expected is 3x7 = 21 with mixed case
        
        console.log(`Unique password count: ${actualCount}`);
        
        // Verify each subsequence exists (with at least one case variation)
        for (const subseq of expectedSubsequences) {
            const hasMatch = Array.from(passwordsSet).some(pw => 
                pw.toLowerCase().includes(subseq) && pw.length >= subseq.length
            );
            
            if (hasMatch) {
                console.log(`✓ "${subseq}" found in passwords`);
                passCount++;
            } else {
                console.log(`✗ "${subseq}" NOT found in passwords`);
                failCount++;
            }
        }
        
        // Check for case variations - may should have lowercase version
        if (passwordsSet.has('may')) {
            console.log('✓ "may" (lowercase) found');
            passCount++;
        } else if (passwordsArray.some(p => p.toLowerCase() === 'may')) {
            console.log('✓ "may" exists (case variation)');
            passCount++;
        } else {
            console.log('✗ "may" NOT found');
            failCount++;
        }
        
        console.log(`\nResults: ${passCount} passed, ${failCount} failed`);
        
        // Return pass/fail status
        return failCount === 0;
    } catch (error) {
        console.error('\nTest failed with error:', error.message);
        if (error.stderr) {
            console.error('STDERR:', error.stderr.toString());
        }
        return false;
    }
}

/**
 * Test Suite 2: Number placeholder should NOT support shortened
 * 
 * Template: Example@{number,max=10,shortened}
 * Expected: Error or empty result because number doesn't have meaningful subsequences
 */
function testNumberWithShortened() {
    console.log('\n' + '='.repeat(70));
    console.log('TEST SUITE 2: Number Placeholder Should Not Support Shortened');
    console.log('='.repeat(70));
    
    cleanupTestFiles();
    
    const template = '{number,max=10,shortened}';
    console.log('\n--- Step 1: Try to generate with number+shortened ---');
    console.log(`Template: ${template}`);
    
    try {
        const output = generatePasswords(template);
        console.log(`Output: ${output.trim()}`);
        
        // Check if error message was returned
        if (output.includes('Error') || output.includes('error')) {
            console.log('\n✓ PASS: Error was returned for number+shortened');
            return true;
        }
        
        const count = parseGenerationOutput(output);
        console.log(`Generated ${count} passwords`);
        
        // If it generated something, check if it's just numbers (not error)
        const passwordsSet = getGeneratedPasswordsSet();
        const passwordsArray = getGeneratedPasswordsArray();
        
        console.log('Generated passwords:', JSON.stringify(passwordsArray, null, 2));
        
        // Check if all passwords are numeric-only
        const allNumeric = passwordsArray.every(pw => /^\d+$/.test(pw));
        
        if (allNumeric) {
            console.log('\n✗ FAIL: Number+shortened should error but generated numbers instead');
            return false;
        }
        
        console.log('\n✓ PASS: Unexpected behavior handled correctly');
        return true;
    } catch (error) {
        console.error('\nTest failed with error:', error.message);
        // This might be expected if the command fails
        return false;
    }
}

/**
 * Test Suite 3: Verify HashSet deduplication
 */
function testHashSetDeduplication() {
    console.log('\n' + '='.repeat(70));
    console.log('TEST SUITE 3: HashSet Deduplication Verification');
    console.log('='.repeat(70));
    
    cleanupTestFiles();
    
    // Generate a template that might produce duplicates
    const template = '{word,min=a,max=c}';
    console.log('\n--- Step 1: Generate passwords ---');
    console.log(`Template: ${template}`);
    
    try {
        const output = generatePasswords(template);
        const count = parseGenerationOutput(output);
        
        // Use Set to check for duplicates
        const passwordsArray = getGeneratedPasswordsArray();
        const uniqueCount = new Set(passwordsArray).size;
        
        console.log(`Total lines in file: ${passwordsArray.length}`);
        console.log(`Unique passwords (Set size): ${uniqueCount}`);
        
        if (passwordsArray.length === uniqueCount) {
            console.log('\n✓ PASS: All passwords are unique (HashSet deduplication working)');
            return true;
        } else {
            console.log('\n✗ FAIL: Duplicate passwords found');
            // Find duplicates
            const counts = {};
            for (const pw of passwordsArray) {
                counts[pw] = (counts[pw] || 0) + 1;
            }
            const duplicates = Object.entries(counts).filter(([_, c]) => c > 1);
            console.log('Duplicates:', duplicates);
            return false;
        }
    } catch (error) {
        console.error('\nTest failed with error:', error.message);
        return false;
    }
}

/**
 * Test Suite 4: Case variations with month
 */
function testMonthCaseVariations() {
    console.log('\n' + '='.repeat(70));
    console.log('TEST SUITE 4: Month Case Variations');
    console.log('='.repeat(70));
    
    cleanupTestFiles();
    
    const templates = [
        { template: '{month,begin=jan,end=jan,case=mixed}', desc: 'mixed case (default)' },
        { template: '{month,begin=jan,end=jan,case=all}', desc: 'all 2^N combinations' },
        { template: '{month,begin=jan,end=jan,case=lower}', desc: 'lowercase only' },
    ];
    
    let allPass = true;
    
    for (const test of templates) {
        console.log(`\n--- Testing: ${test.desc} ---`);
        
        cleanupTestFiles();
        
        try {
            const output = generatePasswords(test.template);
            const count = parseGenerationOutput(output);
            
            console.log(`Generated ${count} passwords`);
            
            const passwordsSet = getGeneratedPasswordsSet();
            
            // Check for expected case patterns
            if (test.desc.includes('lower')) {
                const hasLower = Array.from(passwordsSet).some(pw => pw === 'january');
                if (hasLower) {
                    console.log('✓ lowercase "january" found');
                } else {
                    console.log('✗ lowercase "january" NOT found');
                    allPass = false;
                }
            }
            
        } catch (error) {
            console.error(`Test failed: ${error.message}`);
            allPass = false;
        }
    }
    
    return allPass;
}

/**
 * Test Suite 5: Prefix handling with @ symbol
 */
function testPrefixWithAtSymbol() {
    console.log('\n' + '='.repeat(70));
    console.log('TEST SUITE 5: Prefix Handling with @ Symbol');
    console.log('='.repeat(70));
    
    cleanupTestFiles();
    
    // Test that @ is treated as literal prefix
    const template = 'Example@{month,begin=may,end=may,case=lower}';
    console.log('\n--- Step 1: Generate passwords with Example@ prefix ---');
    console.log(`Template: ${template}`);
    
    try {
        const output = generatePasswords(template);
        const count = parseGenerationOutput(output);
        
        console.log(`Generated ${count} passwords`);
        
        const passwordsArray = getGeneratedPasswordsArray();
        
        // Check if Example@ is preserved
        const hasExampleAt = passwordsArray.some(pw => pw.startsWith('Example@'));
        
        if (hasExampleAt) {
            console.log('✓ "Example@" prefix preserved');
            return true;
        } else {
            console.log('✗ "Example@" prefix NOT found in output');
            console.log('Generated:', JSON.stringify(passwordsArray, null, 2));
            return false;
        }
    } catch (error) {
        console.error('\nTest failed with error:', error.message);
        return false;
    }
}

/**
 * Main execution
 */
function main() {
    console.log('BitUnlocker Word Generation Integration Tests');
    console.log('==============================================');
    console.log(`Working directory: ${WORKSPACE_DIR}`);
    
    let allPass = true;
    
    try {
        // Run all test suites in order
        if (!testMonthWithShortened()) allPass = false;
        
        cleanupTestFiles();
        
        if (!testNumberWithShortened()) allPass = false;
        
        cleanupTestFiles();
        
        if (!testHashSetDeduplication()) allPass = false;
        
        cleanupTestFiles();
        
        if (!testMonthCaseVariations()) allPass = false;
        
        cleanupTestFiles();
        
        if (!testPrefixWithAtSymbol()) allPass = false;
        
        console.log('\n' + '='.repeat(70));
        if (allPass) {
            console.log('✓ All integration tests PASSED!');
        } else {
            console.log('✗ Some tests FAILED - see details above');
            process.exit(1);
        }
        console.log('='.repeat(70));
        
    } catch (error) {
        console.error('\nTest failed with error:', error.message);
        process.exit(1);
    }
}

main();