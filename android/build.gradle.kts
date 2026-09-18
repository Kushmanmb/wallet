buildscript {
    repositories {
        gradlePluginPortal()
        google()
        mavenCentral()
    }
    dependencies {
        classpath(libs.gradle)
        classpath(libs.hilt.android.gradle.plugin)
        classpath(libs.kotlin.serialization)
    }
}

plugins {
    alias(libs.plugins.ksp) apply false
    alias(libs.plugins.android.library) apply false
    alias(libs.plugins.google.services) apply false
    alias(libs.plugins.room) apply false
    alias(libs.plugins.compose.compiler) apply false
}

allprojects {
    repositories {
        google()
        mavenCentral()
        maven { url = uri("https://jitpack.io") }
    }

    dependencyLocking {
        lockAllConfigurations()
    }
}

subprojects {
    configurations.configureEach {
        resolutionStrategy.activateDependencyLocking()
    }
    tasks.withType<Test>().configureEach {
        systemProperty("jna.library.path", File(rootDir, "../core/target/debug").absolutePath)
    }
    listOf("com.android.library", "com.android.application").forEach { pluginId ->
        plugins.withId(pluginId) {
            dependencies.add("testImplementation", "net.java.dev.jna:jna:5.18.1")
        }
    }
}

tasks.register("clean", Delete::class) {
    delete(layout.buildDirectory)
}
