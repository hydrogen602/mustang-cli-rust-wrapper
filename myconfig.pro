# Input and output
-injars Mustang-CLI-2.20.0.jar
-outjars Mustang-CLI-2.20.0-reduced.jar

# Library jars - add all required modules
-libraryjars <java.home>/jmods/java.base.jmod(!**.jar;!module-info.class)
-libraryjars <java.home>/jmods/java.desktop.jmod(!**.jar;!module-info.class)
-libraryjars <java.home>/jmods/java.xml.jmod(!**.jar;!module-info.class)
-libraryjars <java.home>/jmods/java.sql.jmod(!**.jar;!module-info.class)
-libraryjars <java.home>/jmods/java.naming.jmod(!**.jar;!module-info.class)
-libraryjars <java.home>/jmods/java.management.jmod(!**.jar;!module-info.class)
-libraryjars <java.home>/jmods/java.instrument.jmod(!**.jar;!module-info.class)
-libraryjars <java.home>/jmods/java.compiler.jmod(!**.jar;!module-info.class)
-libraryjars <java.home>/jmods/jdk.xml.dom.jmod(!**.jar;!module-info.class)

# Enable shrinking to remove unused code
# Enable optimization (can help reduce size further)
# Obfuscation is optional but can also reduce size slightly

# Keep the main entry point
-keep public class org.mustangproject.commandline.Main {
    public static void main(java.lang.String[]);
}

# Keep classes that might be accessed via reflection (common in Java libraries)
# Adjust these based on what features you actually use
-keepclassmembers class * {
    @org.mustangproject.** *;
}

# Keep Apache Commons Logging classes used via reflection (see manual_reflectconfig.json)
-keep class org.apache.commons.logging.impl.LogFactoryImpl { *; }
-keep class org.apache.commons.logging.LogFactory { *; }
-keep class org.apache.commons.logging.impl.SimpleLog { *; }

# Keep SLF4J API and simple backend (provider is discovered via ServiceLoader)
-keep class org.slf4j.** { *; }

# Keep Eclipse Angus Activation classes (required for native image)
-keep class org.eclipse.angus.activation.** { *; }
-keep class org.eclipse.angus.activation.nativeimage.** { *; }

# Keep native methods
-keepclasseswithmembernames class * {
    native <methods>;
}

# Keep serialization classes if used
-keepclassmembers class * implements java.io.Serializable {
    static final long serialVersionUID;
    private static final java.io.ObjectStreamField[] serialPersistentFields;
    private void writeObject(java.io.ObjectOutputStream);
    private void readObject(java.io.ObjectInputStream);
    java.lang.Object writeReplace();
    java.lang.Object readResolve();
}

# Keep resource files that might be needed (fonts, configs, etc.)
# Comment out if you don't need fonts/resources
# Note: ProGuard keeps resources by default, but you can filter them
# If you don't need fonts, add: -adaptresourcefilecontents **/*.ttf

# Don't warn about missing classes in library jars
-dontwarn **

# Print configuration for debugging
-verbose
