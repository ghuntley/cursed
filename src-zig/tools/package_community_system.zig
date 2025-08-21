// CURSED Package Community System
// Ratings, reviews, and community feedback features

const std = @import("std");
const ArrayList = std.ArrayList;
const HashMap = std.HashMap;
const Allocator = std.mem.Allocator;
const print = std.debug.print;
const registry = @import("package_registry_advanced.zig");

// ===== Community System =====

pub const CommunitySystem = struct {
    allocator: Allocator,
    reviews: HashMap([]const u8, ArrayList(Review), std.hash_map.StringContext, 80),
    ratings: HashMap([]const u8, RatingStats, std.hash_map.StringContext, 80),
    user_profiles: HashMap([]const u8, UserProfile, std.hash_map.StringContext, 80),
    moderation_queue: ArrayList(ModerationItem),
    
    pub fn init(allocator: Allocator) CommunitySystem {
        return CommunitySystem{
            .allocator = allocator,
            .reviews = HashMap([]const u8, ArrayList(Review), std.hash_map.StringContext, 80).init(allocator),
            .ratings = HashMap([]const u8, RatingStats, std.hash_map.StringContext, 80).init(allocator),
            .user_profiles = HashMap([]const u8, UserProfile, std.hash_map.StringContext, 80).init(allocator),
            .moderation_queue = ArrayList(ModerationItem).init(allocator),
        };
    }
    
    pub fn deinit(self: *CommunitySystem) void {
        // Clean up reviews
        var review_iter = self.reviews.iterator();
        while (review_iter.next()) |entry| {
            for (entry.value_ptr.items) |*review| {
                review.deinit(allocator);
            }
            entry.value_ptr.deinit(allocator);
        }
        self.reviews.deinit(allocator);
        
        // Clean up user profiles
        var profile_iter = self.user_profiles.iterator();
        while (profile_iter.next()) |entry| {
            entry.value_ptr.deinit(allocator);
        }
        self.user_profiles.deinit(allocator);
        
        self.ratings.deinit(allocator);
        self.moderation_queue.deinit(allocator);
    }
    
    // ===== Review System =====
    
    pub fn submitReview(self: *CommunitySystem, review_data: ReviewSubmission) !ReviewResult {
        print("📝 Submitting review for {s} by {s}\n", .{review_data.package_name, review_data.author});
        
        // Validate review
        const validation = try self.validateReview(review_data);
        if (!validation.valid) {
            return ReviewResult{
                .success = false,
                .review_id = null,
                .message = validation.message,
            };
        }
        
        // Check if user can review this package
        const auth_check = try self.checkReviewPermissions(review_data.author, review_data.package_name);
        if (!auth_check.allowed) {
            return ReviewResult{
                .success = false,
                .review_id = null,
                .message = auth_check.reason,
            };
        }
        
        // Create review
        const review_id = try self.generateReviewId();
        var review = Review{
            .id = review_id,
            .package_name = try self.allocator.dupe(u8, review_data.package_name),
            .author = try self.allocator.dupe(u8, review_data.author),
            .rating = review_data.rating,
            .title = try self.allocator.dupe(u8, review_data.title),
            .content = try self.allocator.dupe(u8, review_data.content),
            .created_at = std.time.timestamp(),
            .updated_at = std.time.timestamp(),
            .helpful_votes = 0,
            .total_votes = 0,
            .verified_download = auth_check.verified_download,
            .status = .pending_moderation,
            .tags = ArrayList([]const u8).init(self.allocator),
        };
        
        // Add to moderation queue if needed
        if (review_data.rating <= 2 or review_data.content.len > 500) {
            const mod_item = ModerationItem{
                .item_type = .review,
                .item_id = review_id,
                .package_name = try self.allocator.dupe(u8, review_data.package_name),
                .author = try self.allocator.dupe(u8, review_data.author),
                .priority = if (review_data.rating == 1) .high else .normal,
                .created_at = std.time.timestamp(),
            };
            try self.moderation_queue.append(mod_item);
            print("   📋 Review queued for moderation\n", .{});
        } else {
            review.status = .approved;
        }
        
        // Store review
        const package_key = try self.allocator.dupe(u8, review_data.package_name);
        if (self.reviews.getPtr(package_key)) |package_reviews| {
            try package_reviews.append(review);
        } else {
            var new_reviews = ArrayList(Review).init(self.allocator);
            try new_reviews.append(review);
            try self.reviews.put(package_key, new_reviews);
        }
        
        // Update rating statistics
        try self.updateRatingStats(review_data.package_name, review_data.rating);
        
        // Update user profile
        try self.updateUserReviewCount(review_data.author);
        
        print("✅ Review submitted successfully (ID: {s})\n", .{review_id});
        
        return ReviewResult{
            .success = true,
            .review_id = review_id,
            .message = if (review.status == .pending_moderation) 
                "Review submitted and queued for moderation" else "Review published successfully",
        };
    }
    
    fn validateReview(self: *CommunitySystem, review_data: ReviewSubmission) !ValidationResult {
        // Rating validation
        if (review_data.rating < 1 or review_data.rating > 5) {
            return ValidationResult{
                .valid = false,
                .message = try self.allocator.dupe(u8, "Rating must be between 1 and 5 stars"),
            };
        }
        
        // Title validation
        if (review_data.title.len < 5) {
            return ValidationResult{
                .valid = false,
                .message = try self.allocator.dupe(u8, "Review title must be at least 5 characters"),
            };
        }
        
        if (review_data.title.len > 100) {
            return ValidationResult{
                .valid = false,
                .message = try self.allocator.dupe(u8, "Review title must be less than 100 characters"),
            };
        }
        
        // Content validation
        if (review_data.content.len < 20) {
            return ValidationResult{
                .valid = false,
                .message = try self.allocator.dupe(u8, "Review content must be at least 20 characters"),
            };
        }
        
        if (review_data.content.len > 2000) {
            return ValidationResult{
                .valid = false,
                .message = try self.allocator.dupe(u8, "Review content must be less than 2000 characters"),
            };
        }
        
        // Content quality checks
        if (self.containsSpam(review_data.content)) {
            return ValidationResult{
                .valid = false,
                .message = try self.allocator.dupe(u8, "Review content appears to be spam"),
            };
        }
        
        return ValidationResult{
            .valid = true,
            .message = try self.allocator.dupe(u8, "Review validation passed"),
        };
    }
    
    fn containsSpam(self: *CommunitySystem, content: []const u8) bool {
        const spam_patterns = [_][]const u8{
            "buy now",
            "click here", 
            "free download",
            "www.",
            "http://",
            "https://",
        };
        
        const lower_content = std.ascii.allocLowerString(self.allocator, content) catch return false;
        defer self.allocator.free(lower_content);
        
        var spam_count: u32 = 0;
        for (spam_patterns) |pattern| {
            if (std.mem.indexOf(u8, lower_content, pattern) != null) {
                spam_count += 1;
            }
        }
        
        return spam_count >= 2; // Multiple spam indicators
    }
    
    fn checkReviewPermissions(self: *CommunitySystem, author: []const u8, package_name: []const u8) !AuthorizationResult {
        _ = package_name;
        
        // Check if user exists and is in good standing
        if (self.user_profiles.get(author)) |profile| {
            if (profile.status == .banned) {
                return AuthorizationResult{
                    .allowed = false,
                    .verified_download = false,
                    .reason = try self.allocator.dupe(u8, "User account is banned"),
                };
            }
            
            if (profile.review_count >= 50 and profile.helpful_ratio < 0.3) {
                return AuthorizationResult{
                    .allowed = false,
                    .verified_download = false,
                    .reason = try self.allocator.dupe(u8, "User has low helpful review ratio"),
                };
            }
            
            // Simulate download verification
            const verified = profile.downloads.items.len > 0; // Simplified check
            
            return AuthorizationResult{
                .allowed = true,
                .verified_download = verified,
                .reason = try self.allocator.dupe(u8, "User authorized to review"),
            };
        } else {
            // Create new user profile
            const new_profile = UserProfile.init(self.allocator, author);
            try self.user_profiles.put(try self.allocator.dupe(u8, author), new_profile);
            
            return AuthorizationResult{
                .allowed = true,
                .verified_download = false,
                .reason = try self.allocator.dupe(u8, "New user authorized to review"),
            };
        }
    }
    
    fn generateReviewId(self: *CommunitySystem) ![]const u8 {
        const timestamp = std.time.timestamp();
        const random_part = std.crypto.random.int(u32);
        return try std.fmt.allocPrint(self.allocator, "review_{x}_{x}", .{timestamp, random_part});
    }
    
    fn updateRatingStats(self: *CommunitySystem, package_name: []const u8, rating: u8) !void {
        const package_key = try self.allocator.dupe(u8, package_name);
        
        if (self.ratings.getPtr(package_key)) |stats| {
            stats.total_ratings += 1;
            stats.rating_sum += rating;
            stats.average_rating = @as(f32, @floatFromInt(stats.rating_sum)) / @as(f32, @floatFromInt(stats.total_ratings));
            
            // Update rating distribution
            stats.rating_distribution[rating - 1] += 1;
        } else {
            var new_stats = RatingStats.init();
            new_stats.total_ratings = 1;
            new_stats.rating_sum = rating;
            new_stats.average_rating = @as(f32, @floatFromInt(rating));
            new_stats.rating_distribution[rating - 1] = 1;
            
            try self.ratings.put(package_key, new_stats);
        }
    }
    
    fn updateUserReviewCount(self: *CommunitySystem, author: []const u8) !void {
        if (self.user_profiles.getPtr(author)) |profile| {
            profile.review_count += 1;
        }
    }
    
    // ===== Rating System =====
    
    pub fn voteOnReview(self: *CommunitySystem, review_id: []const u8, voter: []const u8, helpful: bool) !VoteResult {
        print("🗳️  {s} voting on review {s}: {s}\n", .{voter, review_id, if (helpful) "helpful" else "not helpful"});
        
        // Find the review
        var found_review: ?*Review = null;
        var review_iter = self.reviews.iterator();
        while (review_iter.next()) |entry| {
            for (entry.value_ptr.items) |*review| {
                if (std.mem.eql(u8, review.id, review_id)) {
                    found_review = review;
                    break;
                }
            }
            if (found_review != null) break;
        }
        
        if (found_review == null) {
            return VoteResult{
                .success = false,
                .message = try self.allocator.dupe(u8, "Review not found"),
            };
        }
        
        const review = found_review.?;
        
        // Check if user already voted
        if (self.hasUserVoted(review_id, voter)) {
            return VoteResult{
                .success = false,
                .message = try self.allocator.dupe(u8, "User has already voted on this review"),
            };
        }
        
        // Record vote
        review.total_votes += 1;
        if (helpful) {
            review.helpful_votes += 1;
        }
        
        // Update user profile helpful ratio
        if (self.user_profiles.getPtr(review.author)) |profile| {
            profile.total_votes_received += 1;
            if (helpful) {
                profile.helpful_votes_received += 1;
            }
            profile.helpful_ratio = @as(f32, @floatFromInt(profile.helpful_votes_received)) / 
                                  @as(f32, @floatFromInt(profile.total_votes_received));
        }
        
        print("✅ Vote recorded successfully\n", .{});
        
        return VoteResult{
            .success = true,
            .message = try self.allocator.dupe(u8, "Vote recorded successfully"),
        };
    }
    
    fn hasUserVoted(self: *CommunitySystem, review_id: []const u8, voter: []const u8) bool {
        // Simplified implementation - in production would track individual votes
        _ = self;
        _ = review_id;
        _ = voter;
        return false; // Allow multiple votes for demo
    }
    
    // ===== Review Display =====
    
    pub fn getReviews(self: *CommunitySystem, package_name: []const u8, options: ReviewDisplayOptions) !ReviewList {
        print("📚 Fetching reviews for {s}\n", .{package_name});
        
        var result = ReviewList.init(self.allocator);
        
        if (self.reviews.get(package_name)) |package_reviews| {
            // Filter reviews
            var filtered_reviews = ArrayList(*Review).init(self.allocator);
            defer filtered_reviews.deinit(allocator);
            
            for (package_reviews.items) |*review| {
                if (review.status != .approved and !options.include_pending) continue;
                if (options.min_rating) |min| if (review.rating < min) continue;
                if (options.verified_only and !review.verified_download) continue;
                
                try filtered_reviews.append(review);
            }
            
            // Sort reviews
            switch (options.sort_by) {
                .newest => self.sortReviewsByDate(filtered_reviews.items, false),
                .oldest => self.sortReviewsByDate(filtered_reviews.items, true),
                .highest_rated => self.sortReviewsByRating(filtered_reviews.items, false),
                .lowest_rated => self.sortReviewsByRating(filtered_reviews.items, true),
                .most_helpful => self.sortReviewsByHelpfulness(filtered_reviews.items),
            }
            
            // Apply pagination
            const start = @min(options.offset, filtered_reviews.items.len);
            const end = @min(start + options.limit, filtered_reviews.items.len);
            
            for (filtered_reviews.items[start..end]) |review| {
                try result.reviews.append(review.*);
            }
            
            result.total_count = @intCast(filtered_reviews.items.len);
        }
        
        // Get rating statistics
        if (self.ratings.get(package_name)) |stats| {
            result.rating_stats = stats;
        }
        
        print("✅ Found {} reviews\n", .{result.reviews.items.len});
        return result;
    }
    
    fn sortReviewsByDate(self: *CommunitySystem, reviews: []*Review, ascending: bool) void {
        _ = self;
        std.sort.pdq(*Review, reviews, {}, struct {
            fn lessThan(context: void, a: *Review, b: *Review) bool {
                _ = context;
                return if (ascending) a.created_at < b.created_at else a.created_at > b.created_at;
            }
        }.lessThan);
    }
    
    fn sortReviewsByRating(self: *CommunitySystem, reviews: []*Review, ascending: bool) void {
        _ = self;
        std.sort.pdq(*Review, reviews, {}, struct {
            fn lessThan(context: void, a: *Review, b: *Review) bool {
                _ = context;
                return if (ascending) a.rating < b.rating else a.rating > b.rating;
            }
        }.lessThan);
    }
    
    fn sortReviewsByHelpfulness(self: *CommunitySystem, reviews: []*Review) void {
        _ = self;
        std.sort.pdq(*Review, reviews, {}, struct {
            fn lessThan(context: void, a: *Review, b: *Review) bool {
                _ = context;
                const a_ratio = if (a.total_votes > 0) @as(f32, @floatFromInt(a.helpful_votes)) / @as(f32, @floatFromInt(a.total_votes)) else 0.0;
                const b_ratio = if (b.total_votes > 0) @as(f32, @floatFromInt(b.helpful_votes)) / @as(f32, @floatFromInt(b.total_votes)) else 0.0;
                return a_ratio > b_ratio; // Descending order
            }
        }.lessThan);
    }
    
    // ===== Moderation System =====
    
    pub fn moderateReview(self: *CommunitySystem, review_id: []const u8, action: ModerationAction, moderator: []const u8) !ModerationResult {
        print("🛡️  {s} moderating review {s}: {s}\n", .{moderator, review_id, @tagName(action)});
        
        // Find review in moderation queue
        var queue_index: ?usize = null;
        for (self.moderation_queue.items, 0..) |item, i| {
            if (std.mem.eql(u8, item.item_id, review_id)) {
                queue_index = i;
                break;
            }
        }
        
        if (queue_index == null) {
            return ModerationResult{
                .success = false,
                .message = try self.allocator.dupe(u8, "Review not found in moderation queue"),
            };
        }
        
        // Find the actual review
        var found_review: ?*Review = null;
        var review_iter = self.reviews.iterator();
        while (review_iter.next()) |entry| {
            for (entry.value_ptr.items) |*review| {
                if (std.mem.eql(u8, review.id, review_id)) {
                    found_review = review;
                    break;
                }
            }
            if (found_review != null) break;
        }
        
        if (found_review == null) {
            return ModerationResult{
                .success = false,
                .message = try self.allocator.dupe(u8, "Review not found"),
            };
        }
        
        const review = found_review.?;
        
        // Apply moderation action
        switch (action) {
            .approve => {
                review.status = .approved;
                print("   ✅ Review approved\n", .{});
            },
            .reject => {
                review.status = .rejected;
                print("   ❌ Review rejected\n", .{});
            },
            .edit => {
                review.status = .edited;
                print("   ✏️  Review marked for editing\n", .{});
            },
            .flag => {
                review.status = .flagged;
                print("   🚩 Review flagged for further review\n", .{});
            },
        }
        
        // Remove from moderation queue
        _ = self.moderation_queue.orderedRemove(queue_index.?);
        
        return ModerationResult{
            .success = true,
            .message = try std.fmt.allocPrint(self.allocator, "Review {s} successfully", .{@tagName(action)}),
        };
    }
    
    pub fn getModerationQueue(self: *CommunitySystem, priority_filter: ?ModerationPriority) !ArrayList(ModerationItem) {
        var filtered_queue = ArrayList(ModerationItem).init(self.allocator);
        
        for (self.moderation_queue.items) |item| {
            if (priority_filter == null or item.priority == priority_filter.?) {
                try filtered_queue.append(item);
            }
        }
        
        return filtered_queue;
    }
    
    // ===== Analytics =====
    
    pub fn getCommunityStats(self: *CommunitySystem, package_name: ?[]const u8) !CommunityStats {
        var stats = CommunityStats.init(self.allocator);
        
        if (package_name) |pkg_name| {
            // Package-specific stats
            if (self.reviews.get(pkg_name)) |package_reviews| {
                stats.total_reviews = @intCast(package_reviews.items.len);
                
                var total_rating: u32 = 0;
                for (package_reviews.items) |review| {
                    if (review.status == .approved) {
                        total_rating += review.rating;
                        stats.approved_reviews += 1;
                    }
                }
                
                if (stats.approved_reviews > 0) {
                    stats.average_rating = @as(f32, @floatFromInt(total_rating)) / @as(f32, @floatFromInt(stats.approved_reviews));
                }
            }
            
            if (self.ratings.get(pkg_name)) |rating_stats| {
                stats.rating_distribution = rating_stats.rating_distribution;
            }
        } else {
            // Global community stats
            var total_reviews: u32 = 0;
            var total_packages: u32 = 0;
            
            var review_iter = self.reviews.iterator();
            while (review_iter.next()) |entry| {
                total_packages += 1;
                total_reviews += @intCast(entry.value_ptr.items.len);
            }
            
            stats.total_reviews = total_reviews;
            stats.total_packages = total_packages;
            stats.total_users = @intCast(self.user_profiles.count());
        }
        
        return stats;
    }
};

// ===== Supporting Types =====

pub const Review = struct {
    id: []const u8,
    package_name: []const u8,
    author: []const u8,
    rating: u8, // 1-5 stars
    title: []const u8,
    content: []const u8,
    created_at: i64,
    updated_at: i64,
    helpful_votes: u32,
    total_votes: u32,
    verified_download: bool,
    status: ReviewStatus,
    tags: ArrayList([]const u8),
    
    pub const ReviewStatus = enum { pending_moderation, approved, rejected, edited, flagged };
    
    pub fn deinit(self: *Review) void {
        self.tags.deinit(allocator);
    }
};

pub const ReviewSubmission = struct {
    package_name: []const u8,
    author: []const u8,
    rating: u8,
    title: []const u8,
    content: []const u8,
};

pub const ReviewResult = struct {
    success: bool,
    review_id: ?[]const u8,
    message: []const u8,
};

pub const ValidationResult = struct {
    valid: bool,
    message: []const u8,
};

pub const AuthorizationResult = struct {
    allowed: bool,
    verified_download: bool,
    reason: []const u8,
};

pub const VoteResult = struct {
    success: bool,
    message: []const u8,
};

pub const RatingStats = struct {
    total_ratings: u32,
    rating_sum: u32,
    average_rating: f32,
    rating_distribution: [5]u32, // 1-star to 5-star counts
    
    pub fn init() RatingStats {
        return RatingStats{
            .total_ratings = 0,
            .rating_sum = 0,
            .average_rating = 0.0,
            .rating_distribution = [_]u32{0} ** 5,
        };
    }
};

pub const UserProfile = struct {
    username: []const u8,
    join_date: i64,
    review_count: u32,
    helpful_votes_received: u32,
    total_votes_received: u32,
    helpful_ratio: f32,
    status: UserStatus,
    downloads: ArrayList([]const u8), // Package names downloaded
    
    pub const UserStatus = enum { active, suspended, banned };
    
    pub fn init(allocator: Allocator, username: []const u8) UserProfile {
        return UserProfile{
            .username = username,
            .join_date = std.time.timestamp(),
            .review_count = 0,
            .helpful_votes_received = 0,
            .total_votes_received = 0,
            .helpful_ratio = 0.0,
            .status = .active,
            .downloads = ArrayList([]const u8).init(allocator),
        };
    }
    
    pub fn deinit(self: *UserProfile) void {
        self.downloads.deinit(allocator);
    }
};

pub const ReviewDisplayOptions = struct {
    sort_by: SortBy = .newest,
    min_rating: ?u8 = null,
    verified_only: bool = false,
    include_pending: bool = false,
    limit: usize = 10,
    offset: usize = 0,
    
    pub const SortBy = enum { newest, oldest, highest_rated, lowest_rated, most_helpful };
};

pub const ReviewList = struct {
    reviews: ArrayList(Review),
    total_count: u32,
    rating_stats: ?RatingStats,
    
    pub fn init(allocator: Allocator) ReviewList {
        return ReviewList{
            .reviews = ArrayList(Review).init(allocator),
            .total_count = 0,
            .rating_stats = null,
        };
    }
    
    pub fn deinit(self: *ReviewList) void {
        for (self.reviews.items) |*review| {
            review.deinit(allocator);
        }
        self.reviews.deinit(allocator);
    }
};

pub const ModerationItem = struct {
    item_type: ItemType,
    item_id: []const u8,
    package_name: []const u8,
    author: []const u8,
    priority: ModerationPriority,
    created_at: i64,
    
    pub const ItemType = enum { review, comment, report };
};

pub const ModerationPriority = enum { low, normal, high, urgent };

pub const ModerationAction = enum { approve, reject, edit, flag };

pub const ModerationResult = struct {
    success: bool,
    message: []const u8,
};

pub const CommunityStats = struct {
    total_reviews: u32,
    approved_reviews: u32,
    total_packages: u32,
    total_users: u32,
    average_rating: f32,
    rating_distribution: [5]u32,
    
    pub fn init(allocator: Allocator) CommunityStats {
        _ = allocator;
        return CommunityStats{
            .total_reviews = 0,
            .approved_reviews = 0,
            .total_packages = 0,
            .total_users = 0,
            .average_rating = 0.0,
            .rating_distribution = [_]u32{0} ** 5,
        };
    }
};

// ===== Test Functions =====

test "review submission validation" {
    const allocator = std.testing.allocator;
    
    var community = CommunitySystem.init(allocator);
    defer community.deinit(allocator);
    
    const invalid_review = ReviewSubmission{
        .package_name = "test-package",
        .author = "test-user",
        .rating = 6, // Invalid rating
        .title = "Bad", // Too short
        .content = "Short", // Too short
    };
    
    const validation = try community.validateReview(invalid_review);
    try std.testing.expect(!validation.valid);
    
    const valid_review = ReviewSubmission{
        .package_name = "test-package",
        .author = "test-user",
        .rating = 5,
        .title = "Great package for JSON parsing",
        .content = "This package is excellent for JSON parsing. It's fast, reliable, and has great documentation.",
    };
    
    const valid_validation = try community.validateReview(valid_review);
    try std.testing.expect(valid_validation.valid);
}

test "rating statistics calculation" {
    const allocator = std.testing.allocator;
    
    var community = CommunitySystem.init(allocator);
    defer community.deinit(allocator);
    
    // Add some ratings
    try community.updateRatingStats("test-package", 5);
    try community.updateRatingStats("test-package", 4);
    try community.updateRatingStats("test-package", 5);
    try community.updateRatingStats("test-package", 3);
    
    const stats = community.ratings.get("test-package").?;
    try std.testing.expect(stats.total_ratings == 4);
    try std.testing.expect(stats.rating_sum == 17);
    try std.testing.expect(stats.average_rating == 4.25);
    try std.testing.expect(stats.rating_distribution[4] == 2); // Two 5-star ratings
}
