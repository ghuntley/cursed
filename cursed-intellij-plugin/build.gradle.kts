plugins {
    id("org.jetbrains.intellij") version "1.15.0"
    kotlin("jvm") version "1.9.0"
    id("org.jetbrains.kotlin.plugin.serialization") version "1.9.0"
}

group = "org.cursed"
version = "2.0.0"

repositories {
    mavenCentral()
}

dependencies {
    implementation("org.jetbrains.kotlinx:kotlinx-serialization-json:1.6.0")
    implementation("com.google.code.gson:gson:2.10.1")
    implementation("org.eclipse.lsp4j:org.eclipse.lsp4j:0.21.0")
    testImplementation("junit:junit:4.13.2")
}

intellij {
    version.set("2023.2")
    type.set("IC") // IntelliJ IDEA Community Edition
    
    plugins.set(listOf(
        "com.intellij.java",
        "org.jetbrains.kotlin",
        "com.intellij.platform.lsp"
    ))
}

tasks {
    withType<JavaCompile> {
        sourceCompatibility = "17"
        targetCompatibility = "17"
    }
    
    withType<org.jetbrains.kotlin.gradle.tasks.KotlinCompile> {
        kotlinOptions.jvmTarget = "17"
    }

    patchPluginXml {
        sinceBuild.set("232")
        untilBuild.set("241.*")
        
        changeNotes.set("""
            <h3>Version 2.0.0</h3>
            <ul>
                <li>Advanced syntax highlighting with semantic tokens</li>
                <li>Comprehensive IntelliSense with type inference</li>
                <li>Integrated debugger with goroutine support</li>
                <li>Project templates and scaffolding</li>
                <li>Real-time error checking and suggestions</li>
                <li>Performance analysis tools</li>
                <li>Cross-compilation support</li>
                <li>Package management integration</li>
            </ul>
        """.trimIndent())
    }

    signPlugin {
        certificateChain.set(System.getenv("CERTIFICATE_CHAIN"))
        privateKey.set(System.getenv("PRIVATE_KEY"))
        password.set(System.getenv("PRIVATE_KEY_PASSWORD"))
    }

    publishPlugin {
        token.set(System.getenv("PUBLISH_TOKEN"))
    }
    
    runIde {
        ideDir.set(file("/opt/idea-IC"))
    }
}

kotlin {
    jvmToolchain(17)
}
